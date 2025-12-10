use cbc::Decryptor;
use cipher::block_padding::Pkcs7;
use cipher::{BlockDecryptMut, KeyIvInit};
use hex::FromHex;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub enum EnvError {
    VarError(env::VarError),
    DecryptionFailed(String),
    MissingKeyFile,
    IoError(std::io::Error),
    KeyFileFormatError(String),
}

impl From<env::VarError> for EnvError {
    fn from(err: env::VarError) -> Self {
        EnvError::VarError(err)
    }
}

impl From<std::io::Error> for EnvError {
    fn from(err: std::io::Error) -> Self {
        EnvError::IoError(err)
    }
}

impl std::fmt::Display for EnvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvError::VarError(e) => write!(f, "Environment variable error: {}", e),
            EnvError::DecryptionFailed(msg) => write!(f, "Failed to decrypt: {}", msg),
            EnvError::MissingKeyFile => write!(f, "Missing key file for decryption"),
            EnvError::IoError(e) => write!(f, "IO error: {}", e),
            EnvError::KeyFileFormatError(msg) => write!(f, "Key file format error: {}", msg),
        }
    }
}

impl Error for EnvError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            EnvError::VarError(e) => Some(e),
            EnvError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

/// Retrieves an environment variable's value.
///
/// # Arguments
///
/// * `name` - The name of the environment variable.
///
/// # Returns
///
/// The value of the environment variable if present, or an error.
pub fn get_var(name: &str) -> Result<String, EnvError> {
    Ok(env::var(name)?)
}

/// Retrieves an environment variable's value or returns a default if not set.
///
/// # Arguments
///
/// * `name` - The name of the environment variable.
/// * `default` - The default value to return if the environment variable is not set.
pub fn get_var_or(name: &str, default: &str) -> String {
    env::var(name).unwrap_or_else(|_| default.to_string())
}

/// Checks if a string slice is encrypted. Encrypted values start with "+encs+".
///
/// # Arguments
///
/// * `value` - The string slice to check.
///
/// # Returns
///
/// `true` if the value is encrypted, `false` otherwise.
pub fn is_encrypted(value: &str) -> bool {
    value.starts_with("+encs+")
}

/// Parses a key file to extract the salt, key, and initialization vector (IV).
///
/// The key file must contain three lines in the format:
/// ```text
/// salt=...
/// key=...
/// iv=...
/// ```
/// Empty or whitespace-only lines are ignored.
///
/// # Arguments
///
/// * `path` - The path to the key file.
///
/// # Returns
///
/// A tuple containing the salt, key, and IV as strings.
fn parse_key_file(path: &str) -> Result<(String, String, String), EnvError> {
    let file = File::open(path).map_err(|_| EnvError::MissingKeyFile)?;
    let reader = BufReader::new(file);

    let mut salt = None;
    let mut key = None;
    let mut iv = None;

    for (line_num, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        let line_num = line_num + 1; // 1-based line numbering for human readability

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

    let salt =
        salt.ok_or_else(|| EnvError::KeyFileFormatError("Missing salt in key file".to_string()))?;
    let key =
        key.ok_or_else(|| EnvError::KeyFileFormatError("Missing key in key file".to_string()))?;
    let iv =
        iv.ok_or_else(|| EnvError::KeyFileFormatError("Missing iv in key file".to_string()))?;

    Ok((salt, key, iv))
}

/// Decrypts an encrypted Geneos environment variable.
///
/// This function assumes the encryption was performed using AES-256 in CBC mode with PKCS7 padding.
/// Encrypted values must start with "+encs+" followed by the hexadecimal representation of the ciphertext.
/// The provided key file must contain the salt, key, and IV in the expected format.
/// If the input value is not encrypted (i.e. does not start with "+encs+"), it is returned unchanged.
///
/// # Arguments
///
/// * `value` - The encrypted string slice.
/// * `key_file` - The path to the key file containing decryption parameters.
///
/// # Returns
///
/// The decrypted string on success, or an error if decryption fails.
///
/// # Example
///
/// ```no_run
/// use geneos_toolkit::env;
///
/// let encrypted = "+encs+69B1E12815FA83702F0016B0E7FBD33B";
/// let decrypted = env::decrypt(encrypted, "path/to/key-file").unwrap();
/// println!("Decrypted value: {}", decrypted);
/// ```
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

/// Retrieves an environment variable and automatically decrypts it if needed.
///
/// If the environment variable's value starts with "+encs+", it is assumed to be encrypted and will
/// be decrypted using the provided key file.
///
/// # Arguments
///
/// * `name` - The name of the environment variable.
/// * `key_file` - The path to the key file containing decryption parameters.
///
/// # Returns
///
/// The plain text value of the environment variable on success, or an error.
pub fn get_secure_var(name: &str, key_file: &str) -> Result<String, EnvError> {
    let value = get_var(name)?;
    if is_encrypted(&value) {
        decrypt(&value, key_file)
    } else {
        Ok(value)
    }
}

/// Retrieves a secure environment variable's value, returning a default if the variable is not set.
///
/// This function first attempts to get the environment variable named `name`.
/// - If the variable is not present, it returns the provided `default` value.
/// - If the variable is present and its value starts with `+encs+`, it is assumed to be encrypted
///   and the function will attempt to decrypt it using the specified `key_file`.
/// - If the variable is present and not encrypted, its value is returned as-is.
///
/// # Errors
///
/// If the variable is present but decryption fails or if any other error occurs (for example,
/// an I/O error or a key file format error), the error is propagated.
///
/// # Example
///
/// ```no_run
/// use geneos_toolkit::env::get_secure_var_or;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let value = get_secure_var_or("SECURE_ENV_VAR", "path/to/key_file", "default_value")?;
///     println!("Value: {}", value);
///     Ok(())
/// }
/// ```
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
    fn test_get_env() {
        // Test retrieving an existing variable.
        with_var("TEST_VAR", Some("test_value"), || {
            assert_eq!(get_var("TEST_VAR").unwrap(), "test_value");
            assert_eq!(get_var_or("TEST_VAR", "default"), "test_value");
        });

        // Test non-existent variable.
        with_var::<_, &str, _, _>("NON_EXISTENT_VAR", None, || {
            assert!(get_var("NON_EXISTENT_VAR").is_err());
            assert_eq!(get_var_or("NON_EXISTENT_VAR", "default"), "default");
        });
    }

    #[test]
    fn test_is_encrypted() {
        assert!(is_encrypted("+encs+1234567890ABCDEF"));
        assert!(!is_encrypted("plain_text"));
        assert!(!is_encrypted(""));
    }

    #[test]
    fn test_parse_key_file() {
        let dir = tempdir().unwrap();
        let key_file_path = dir.path().join("key-file");

        // Create a valid key file.
        {
            let mut file = File::create(&key_file_path).unwrap();
            writeln!(file, "{}", VALID_KEY_FILE_CONTENTS).unwrap();
        }

        // Valid parsing.
        let result = parse_key_file(key_file_path.to_str().unwrap());
        assert!(result.is_ok());
        let (salt, key, iv) = result.unwrap();
        assert_eq!(salt, "89A6A795C9CCECB5");
        assert_eq!(
            key,
            "26D6EDD53A0AFA8FA1AA3FBCD2FFF2A0BF4809A4E04511F629FC732C2A42A8FC"
        );
        assert_eq!(iv, "472A3557ADDD2525AD4E555738636A67");

        // Test invalid key file: unexpected content.
        let invalid_key_file_path = dir.path().join("invalid-key-file");
        let mut file = File::create(&invalid_key_file_path).unwrap();
        writeln!(file, "salt=1234567890ABCDEF").unwrap();
        writeln!(file, "invalid_line=something").unwrap();
        writeln!(file, "iv=1234567890ABCDEF").unwrap();

        let result = parse_key_file(invalid_key_file_path.to_str().unwrap());
        assert!(result.is_err());

        if let Err(EnvError::KeyFileFormatError(msg)) = result {
            assert!(msg.contains("Unexpected content at line 2"));
        } else {
            panic!("Expected KeyFileFormatError");
        }

        // Test empty file.
        let empty_key_file_path = dir.path().join("empty-key-file");
        let _file = File::create(&empty_key_file_path).unwrap();

        let result = parse_key_file(empty_key_file_path.to_str().unwrap());
        assert!(result.is_err());

        if let Err(EnvError::KeyFileFormatError(msg)) = result {
            // An empty file will first fail on the test for a salt.
            assert!(msg.contains("Missing salt in key file"));
        } else {
            panic!("Expected KeyFileFormatError");
        }

        // Test key file with empty/whitespace lines.
        let spaced_key_file_path = dir.path().join("spaced-key-file");
        let mut file = File::create(&spaced_key_file_path).unwrap();
        writeln!(file, "salt=1234567890ABCDEF").unwrap();
        writeln!(file).unwrap();
        writeln!(file, "key=1234567890ABCDEF1234567890ABCDEF").unwrap();
        writeln!(file, "  ").unwrap();
        writeln!(file, "iv=1234567890ABCDEF").unwrap();

        let result = parse_key_file(spaced_key_file_path.to_str().unwrap());
        assert!(result.is_ok());
    }

    #[test]
    fn test_decrypt_unencrypted() {
        let dir = tempdir().unwrap();
        let key_file_path = dir.path().join("key-file");
        {
            let mut file = File::create(&key_file_path).unwrap();
            writeln!(file, "{}", VALID_KEY_FILE_CONTENTS).unwrap();
        }
        // Unencrypted values should pass through unchanged.
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

        // Plain variable.
        with_var("PLAIN_VAR", Some("plain_text"), || {
            let result = get_secure_var("PLAIN_VAR", key_file_path.to_str().unwrap());
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "plain_text");
        });

        // Missing variable.
        with_var::<_, &str, _, _>("MISSING_VAR", None, || {
            let result = get_secure_var("MISSING_VAR", key_file_path.to_str().unwrap());
            assert!(result.is_err());
            if let Err(e) = result {
                assert!(matches!(e, EnvError::VarError(_)));
            }
        });

        // Encrypted variable 1.
        with_var("ENCRYPTED_VAR", Some(ENCRYPTED_VAR_1), || {
            let result = get_secure_var("ENCRYPTED_VAR", key_file_path.to_str().unwrap());
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), DECRYPTED_VAR_1.to_string());
        });

        // Encrypted variable 2.
        with_var("ENCRYPTED_VAR", Some(ENCRYPTED_VAR_2), || {
            let result = get_secure_var("ENCRYPTED_VAR", key_file_path.to_str().unwrap());
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), DECRYPTED_VAR_2.to_string());
        });
    }

    #[test]
    fn test_decrypt_missing_keyfile() {
        let result = decrypt(ENCRYPTED_VAR_1, "/non/existent/keyfile");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EnvError::MissingKeyFile));
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
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EnvError::DecryptionFailed(_)));
    }
}
