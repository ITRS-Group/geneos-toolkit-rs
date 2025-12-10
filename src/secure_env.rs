use crate::env::{get_var, is_encrypted, EnvError};
use cbc::Decryptor;
use cipher::block_padding::Pkcs7;
use cipher::{BlockDecryptMut, KeyIvInit};
use hex::FromHex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_key_file(path: &str) -> Result<(String, String, String), EnvError> {
    let file = File::open(path).map_err(|_| EnvError::MissingKeyFile)?;
    let reader = BufReader::new(file);

    let mut salt = None;
    let mut key = None;
    let mut iv = None;

    for (line_num, line_result) in reader.lines().enumerate() {
        let line = line_result.map_err(EnvError::IoError)?;
        let line_num = line_num + 1;

        if line.trim().is_empty() {
            continue;
        }

        match line.trim().split_once('=') {
            Some(("salt", value)) => salt = Some(value.to_string()),
            Some(("key", value)) => key = Some(value.to_string()),
            Some(("iv", value)) => iv = Some(value.to_string()),
            Some((other, _)) => {
                return Err(EnvError::KeyFileFormatError(format!(
                    "Unexpected content at line {}: '{}'",
                    line_num, other
                )));
            }
            None => {}
        }
    }

    let salt = salt.ok_or_else(|| EnvError::KeyFileFormatError("Missing salt in key file".to_string()))?;
    let key = key.ok_or_else(|| EnvError::KeyFileFormatError("Missing key in key file".to_string()))?;
    let iv = iv.ok_or_else(|| EnvError::KeyFileFormatError("Missing iv in key file".to_string()))?;

    Ok((salt, key, iv))
}

/// Decrypts an encrypted value using AES-256-CBC with PKCS7 padding.
/// Values not prefixed with `+encs+` are returned unchanged.
pub fn decrypt(value: &str, key_file: &str) -> Result<String, EnvError> {
    if value.len() < 6 || !is_encrypted(value) {
        return Ok(value.to_string());
    }

    let hex = &value[6..];
    let mut encrypted_bytes = Vec::from_hex(hex)
        .map_err(|e| EnvError::DecryptionFailed(format!("Invalid hex encoding: {}", e)))?;

    let (_, key_hex, iv_hex) = parse_key_file(key_file)?;

    let key_bytes = Vec::from_hex(key_hex)
        .map_err(|e| EnvError::DecryptionFailed(format!("Invalid key hex: {}", e)))?;
    let iv_bytes = Vec::from_hex(iv_hex)
        .map_err(|e| EnvError::DecryptionFailed(format!("Invalid iv hex: {}", e)))?;

    type Aes256Cbc = Decryptor<aes::Aes256>;

    let decrypted_bytes = Aes256Cbc::new_from_slices(&key_bytes, &iv_bytes)
        .map_err(|_| EnvError::DecryptionFailed("Invalid key or IV length".to_string()))?
        .decrypt_padded_mut::<Pkcs7>(&mut encrypted_bytes)
        .map_err(|e| EnvError::DecryptionFailed(format!("Decryption failed: {}", e)))?;

    String::from_utf8(decrypted_bytes.into())
        .map_err(|e| EnvError::DecryptionFailed(format!("Invalid UTF-8 in decrypted data: {}", e)))
}

/// Retrieves an environment variable and decrypts it if it is encrypted.
pub fn get_secure_var(name: &str, key_file: &str) -> Result<String, EnvError> {
    let value = get_var(name)?;
    if is_encrypted(&value) {
        decrypt(&value, key_file)
    } else {
        Ok(value)
    }
}

/// Retrieves a secure environment variable, returning a default if it is missing.
pub fn get_secure_var_or(name: &str, key_file: &str, default: &str) -> Result<String, EnvError> {
    match get_var(name) {
        Ok(val) => {
            if is_encrypted(&val) {
                decrypt(&val, key_file)
            } else {
                Ok(val)
            }
        }
        Err(EnvError::VarError(_)) => Ok(default.to_string()),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use temp_env::with_var;
    use tempfile::tempdir;

    const VALID_KEY_FILE_CONTENTS: &str = r#"salt=89A6A795C9CCECB5
key=26D6EDD53A0AFA8FA1AA3FBCD2FFF2A0BF4809A4E04511F629FC732C2A42A8FC
iv=472A3557ADDD2525AD4E555738636A67
"#;

    const ENCRYPTED_VAR_1: &str = "+encs+BCC9E963342C9CFEFB45093F3437A680";
    const DECRYPTED_VAR_1: &str = "12345";

    const ENCRYPTED_VAR_2: &str = "+encs+3510EEEF4163EB21C671FB5C57ADFCE2";
    const DECRYPTED_VAR_2: &str = "/";

    #[test]
    fn test_parse_key_file() {
        let dir = tempdir().unwrap();
        let key_file_path = dir.path().join("key-file");

        {
            let mut file = File::create(&key_file_path).unwrap();
            writeln!(file, "{}", VALID_KEY_FILE_CONTENTS).unwrap();
        }

        let result = parse_key_file(key_file_path.to_str().unwrap());
        assert!(result.is_ok());
        let (salt, key, iv) = result.unwrap();
        assert_eq!(salt, "89A6A795C9CCECB5");
        assert_eq!(
            key,
            "26D6EDD53A0AFA8FA1AA3FBCD2FFF2A0BF4809A4E04511F629FC732C2A42A8FC"
        );
        assert_eq!(iv, "472A3557ADDD2525AD4E555738636A67");

        // invalid file
        let invalid_key_file_path = dir.path().join("invalid-key-file");
        let mut file = File::create(&invalid_key_file_path).unwrap();
        writeln!(file, "salt=1234567890ABCDEF").unwrap();
        writeln!(file, "invalid_line=something").unwrap();
        writeln!(file, "iv=1234567890ABCDEF").unwrap();
        let result = parse_key_file(invalid_key_file_path.to_str().unwrap());
        assert!(matches!(result, Err(EnvError::KeyFileFormatError(_))));
    }

    #[test]
    fn test_decrypt_unencrypted() {
        let dir = tempdir().unwrap();
        let key_file_path = dir.path().join("key-file");
        {
            let mut file = File::create(&key_file_path).unwrap();
            writeln!(file, "{}", VALID_KEY_FILE_CONTENTS).unwrap();
        }
        let result = decrypt("not-encrypted", key_file_path.to_str().unwrap());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "not-encrypted");
    }

    #[test]
    fn test_get_secure() {
        let dir = tempdir().unwrap();
        let key_file_path = dir.path().join("key-file");
        {
            let mut file = File::create(&key_file_path).unwrap();
            writeln!(file, "{}", VALID_KEY_FILE_CONTENTS).unwrap();
        }

        with_var("PLAIN_VAR", Some("plain_text"), || {
            let result = get_secure_var("PLAIN_VAR", key_file_path.to_str().unwrap());
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "plain_text");
        });

        with_var::<_, &str, _, _>("MISSING_VAR", None, || {
            let result = get_secure_var("MISSING_VAR", key_file_path.to_str().unwrap());
            assert!(result.is_err());
            if let Err(e) = result {
                assert!(matches!(e, EnvError::VarError(_)));
            }
        });

        with_var("ENCRYPTED_VAR", Some(ENCRYPTED_VAR_1), || {
            let result = get_secure_var("ENCRYPTED_VAR", key_file_path.to_str().unwrap());
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), DECRYPTED_VAR_1.to_string());
        });

        with_var("ENCRYPTED_VAR", Some(ENCRYPTED_VAR_2), || {
            let result = get_secure_var("ENCRYPTED_VAR", key_file_path.to_str().unwrap());
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), DECRYPTED_VAR_2.to_string());
        });
    }

    #[test]
    fn test_decrypt_missing_keyfile() {
        let result = decrypt(ENCRYPTED_VAR_1, "/non/existent/keyfile");
        assert!(matches!(result, Err(EnvError::MissingKeyFile)));
    }

    #[test]
    fn test_decrypt_invalid_hex() {
        let dir = tempdir().unwrap();
        let key_file_path = dir.path().join("key-file");
        {
            let mut file = File::create(&key_file_path).unwrap();
            writeln!(file, "{}", VALID_KEY_FILE_CONTENTS).unwrap();
        }
        let result = decrypt("+encs+ZZ", key_file_path.to_str().unwrap());
        assert!(matches!(result, Err(EnvError::DecryptionFailed(_))));
    }
}
