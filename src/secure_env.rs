use crate::env::{EnvError, get_var, is_encrypted};
use cbc::Decryptor;
use cipher::block_padding::Pkcs7;
use cipher::{BlockDecryptMut, KeyIvInit};
use hex::FromHex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use zeroize::Zeroizing;

const MAX_KEY_FILE_SIZE: u64 = 1024;

type KeyComponents = (Zeroizing<String>, Zeroizing<String>, Zeroizing<String>);

fn parse_key_file(path: &str) -> Result<KeyComponents, EnvError> {
    parse_key_file_with_opener(path, open_key_file)
}

fn open_key_file(path: &str) -> io::Result<File> {
    File::open(path)
}

fn parse_key_file_with_opener(
    path: &str,
    open: impl FnOnce(&str) -> io::Result<File>,
) -> Result<KeyComponents, EnvError> {
    let file = open(path)
        .map_err(|err| EnvError::IoError(io::Error::new(err.kind(), "cannot open key file")))?;
    parse_open_key_file(file)
}

fn parse_open_key_file(file: File) -> Result<KeyComponents, EnvError> {
    validate_key_file_metadata(&file)?;
    let reader = BufReader::new(file);

    let mut salt: Option<Zeroizing<String>> = None;
    let mut key: Option<Zeroizing<String>> = None;
    let mut iv: Option<Zeroizing<String>> = None;

    for (line_num, line_result) in reader.lines().enumerate() {
        let line = Zeroizing::new(line_result.map_err(EnvError::IoError)?);
        let line_num = line_num + 1;

        if line.trim().is_empty() {
            continue;
        }

        match line.trim().split_once('=') {
            Some(("salt", value)) => {
                if salt.is_some() {
                    return Err(EnvError::KeyFileFormatError(
                        "duplicate salt in key file".to_string(),
                    ));
                }
                salt = Some(Zeroizing::new(value.to_string()));
            }
            Some(("key", value)) => {
                if key.is_some() {
                    return Err(EnvError::KeyFileFormatError(
                        "duplicate key in key file".to_string(),
                    ));
                }
                key = Some(Zeroizing::new(value.to_string()));
            }
            Some(("iv", value)) => {
                if iv.is_some() {
                    return Err(EnvError::KeyFileFormatError(
                        "duplicate iv in key file".to_string(),
                    ));
                }
                iv = Some(Zeroizing::new(value.to_string()));
            }
            Some((_, _)) => {
                return Err(EnvError::KeyFileFormatError(format!(
                    "unexpected key at line {} in key file",
                    line_num
                )));
            }
            None => {
                return Err(EnvError::KeyFileFormatError(format!(
                    "invalid line {} in key file",
                    line_num
                )));
            }
        }
    }

    let salt =
        salt.ok_or_else(|| EnvError::KeyFileFormatError("incomplete key file".to_string()))?;
    let key = key.ok_or_else(|| EnvError::KeyFileFormatError("incomplete key file".to_string()))?;
    let iv = iv.ok_or_else(|| EnvError::KeyFileFormatError("incomplete key file".to_string()))?;

    Ok((salt, key, iv))
}

fn validate_key_file_metadata(file: &File) -> Result<(), EnvError> {
    let meta = file
        .metadata()
        .map_err(|err| EnvError::IoError(io::Error::new(err.kind(), "cannot open key file")))?;

    if meta.len() > MAX_KEY_FILE_SIZE {
        return Err(EnvError::KeyFileFormatError(
            "key file too large".to_string(),
        ));
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if meta.permissions().mode() & 0o004 != 0 {
            return Err(EnvError::KeyFileFormatError(
                "key file is world-readable".to_string(),
            ));
        }
    }

    Ok(())
}

/// Decrypts an encrypted value using AES-256-CBC with PKCS7 padding.
/// Values not prefixed with `+encs+` are returned unchanged.
///
/// Returns `Zeroizing<String>` so the decrypted secret is automatically
/// zeroed when dropped. Callers can still use `&str` via auto-deref.
pub fn decrypt(value: &str, key_file: &str) -> Result<Zeroizing<String>, EnvError> {
    if !is_encrypted(value) {
        return Ok(Zeroizing::new(value.to_string()));
    }

    let hex = &value[6..];
    if hex.is_empty() {
        return Err(EnvError::DecryptionFailed("decryption failed".to_string()));
    }

    let mut encrypted_bytes = Zeroizing::new(
        Vec::from_hex(hex)
            .map_err(|_| EnvError::DecryptionFailed("decryption failed".to_string()))?,
    );

    // Salt was consumed during PBKDF key derivation by Geneos Gateway;
    // only key and IV are needed for decryption.
    let (_, key_hex, iv_hex) = parse_key_file(key_file)?;

    let key_bytes = Zeroizing::new(
        Vec::from_hex(&*key_hex)
            .map_err(|_| EnvError::DecryptionFailed("decryption failed".to_string()))?,
    );
    let iv_bytes = Zeroizing::new(
        Vec::from_hex(&*iv_hex)
            .map_err(|_| EnvError::DecryptionFailed("decryption failed".to_string()))?,
    );

    type Aes256Cbc = Decryptor<aes::Aes256>;

    let len = {
        let decrypted = Aes256Cbc::new_from_slices(&key_bytes, &iv_bytes)
            .map_err(|_| EnvError::DecryptionFailed("decryption failed".to_string()))?
            .decrypt_padded_mut::<Pkcs7>(&mut encrypted_bytes)
            .map_err(|_| EnvError::DecryptionFailed("decryption failed".to_string()))?;
        decrypted.len()
    };

    encrypted_bytes.truncate(len);
    let raw = std::mem::take(&mut *encrypted_bytes);

    match String::from_utf8(raw) {
        Ok(s) => Ok(Zeroizing::new(s)),
        Err(e) => {
            drop(Zeroizing::new(e.into_bytes()));
            Err(EnvError::DecryptionFailed("decryption failed".to_string()))
        }
    }
}

/// Retrieves an environment variable and decrypts it if it is encrypted.
///
/// Returns `Zeroizing<String>` — the value is zeroed on drop whether or
/// not it was encrypted.
pub fn get_secure_var(name: &str, key_file: &str) -> Result<Zeroizing<String>, EnvError> {
    let value = get_var(name)?;
    if is_encrypted(&value) {
        decrypt(&value, key_file)
    } else {
        Ok(Zeroizing::new(value))
    }
}

/// Retrieves a secure environment variable, returning a default if it is missing.
///
/// Returns `Zeroizing<String>` — the value is zeroed on drop whether or
/// not it was encrypted.
pub fn get_secure_var_or(
    name: &str,
    key_file: &str,
    default: &str,
) -> Result<Zeroizing<String>, EnvError> {
    match get_var(name) {
        Ok(val) => {
            if is_encrypted(&val) {
                decrypt(&val, key_file)
            } else {
                Ok(Zeroizing::new(val))
            }
        }
        Err(EnvError::VarError(std::env::VarError::NotPresent)) => {
            Ok(Zeroizing::new(default.to_string()))
        }
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

    // TEST KEY MATERIAL ONLY - never use these keys or ciphertexts in production.
    const VALID_KEY_FILE_CONTENTS: &str = r#"salt=89A6A795C9CCECB5
key=26D6EDD53A0AFA8FA1AA3FBCD2FFF2A0BF4809A4E04511F629FC732C2A42A8FC
iv=472A3557ADDD2525AD4E555738636A67
"#;

    fn write_key_file(path: &std::path::Path, contents: &str) {
        {
            let mut file = File::create(path).unwrap();
            writeln!(file, "{}", contents).unwrap();
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600)).unwrap();
        }
    }

    const ENCRYPTED_VAR_1: &str = "+encs+BCC9E963342C9CFEFB45093F3437A680";
    const DECRYPTED_VAR_1: &str = "12345";

    const ENCRYPTED_VAR_2: &str = "+encs+3510EEEF4163EB21C671FB5C57ADFCE2";
    const DECRYPTED_VAR_2: &str = "/";

    #[cfg(unix)]
    fn invalid_secret_value() -> std::ffi::OsString {
        use std::os::unix::ffi::OsStringExt;

        std::ffi::OsString::from_vec(b"secure-secret-\xFF-token".to_vec())
    }

    fn assert_key_file_format_error(err: &EnvError, expected_inner: &str) {
        match err {
            EnvError::KeyFileFormatError(inner) => assert!(
                inner.contains(expected_inner),
                "expected inner key-file error to contain {expected_inner:?}, got: {inner}"
            ),
            other => panic!("expected KeyFileFormatError, got {other:?}"),
        }

        let display = err.to_string();
        assert_eq!(display, "Key file format error");
        assert!(
            !display.contains(expected_inner),
            "Display must not expose inner key-file details, got: {display}"
        );
    }

    #[cfg(unix)]
    fn assert_notunicode_error_redacted(err: &EnvError) {
        assert!(
            matches!(err, EnvError::VarError(std::env::VarError::NotUnicode(_))),
            "expected NotUnicode EnvError, got {err:?}"
        );
        assert_eq!(
            err.to_string(),
            "Environment variable error: environment variable value is not valid Unicode"
        );
        assert_eq!(format!("{err:?}"), "VarError(NotUnicode([REDACTED]))");
        assert!(
            std::error::Error::source(err).is_none(),
            "NotUnicode source should not expose std::env::VarError"
        );

        let formatted = format!("{err} {err:?}");
        assert!(!formatted.contains("secure-secret"));
        assert!(!formatted.contains("token"));
    }

    #[test]
    fn test_parse_key_file() {
        let dir = tempdir().unwrap();
        let key_file_path = dir.path().join("key-file");

        write_key_file(&key_file_path, VALID_KEY_FILE_CONTENTS);

        let result = parse_key_file(key_file_path.to_str().unwrap());
        assert!(result.is_ok());
        let (salt, key, iv) = result.unwrap();
        assert_eq!(&*salt, "89A6A795C9CCECB5");
        assert_eq!(
            &*key,
            "26D6EDD53A0AFA8FA1AA3FBCD2FFF2A0BF4809A4E04511F629FC732C2A42A8FC"
        );
        assert_eq!(&*iv, "472A3557ADDD2525AD4E555738636A67");

        // invalid file
        let invalid_key_file_path = dir.path().join("invalid-key-file");
        write_key_file(
            &invalid_key_file_path,
            "salt=1234567890ABCDEF\ninvalid_line=something\niv=1234567890ABCDEF",
        );
        let result = parse_key_file(invalid_key_file_path.to_str().unwrap());
        assert!(matches!(result, Err(EnvError::KeyFileFormatError(_))));
    }

    #[test]
    fn test_decrypt_unencrypted() {
        let dir = tempdir().unwrap();
        let key_file_path = dir.path().join("key-file");
        write_key_file(&key_file_path, VALID_KEY_FILE_CONTENTS);
        let result = decrypt("not-encrypted", key_file_path.to_str().unwrap());
        assert!(result.is_ok());
        assert_eq!(&*result.unwrap(), "not-encrypted");
    }

    #[test]
    fn test_get_secure() {
        let dir = tempdir().unwrap();
        let key_file_path = dir.path().join("key-file");
        write_key_file(&key_file_path, VALID_KEY_FILE_CONTENTS);

        with_var("PLAIN_VAR", Some("plain_text"), || {
            let result = get_secure_var("PLAIN_VAR", key_file_path.to_str().unwrap());
            assert!(result.is_ok());
            assert_eq!(&*result.unwrap(), "plain_text");
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
            assert_eq!(&*result.unwrap(), DECRYPTED_VAR_1);
        });

        with_var("ENCRYPTED_VAR", Some(ENCRYPTED_VAR_2), || {
            let result = get_secure_var("ENCRYPTED_VAR", key_file_path.to_str().unwrap());
            assert!(result.is_ok());
            assert_eq!(&*result.unwrap(), DECRYPTED_VAR_2);
        });
    }

    #[test]
    fn test_get_secure_var_or() {
        let dir = tempdir().unwrap();
        let key_file_path = dir.path().join("key-file");
        write_key_file(&key_file_path, VALID_KEY_FILE_CONTENTS);

        with_var::<_, &str, _, _>("MISSING_VAR_OR", None, || {
            let result = get_secure_var_or(
                "MISSING_VAR_OR",
                key_file_path.to_str().unwrap(),
                "fallback",
            );
            assert!(result.is_ok());
            assert_eq!(&*result.unwrap(), "fallback");
        });

        with_var("ENCRYPTED_VAR_OR", Some(ENCRYPTED_VAR_1), || {
            let result = get_secure_var_or(
                "ENCRYPTED_VAR_OR",
                key_file_path.to_str().unwrap(),
                "fallback",
            );
            assert!(result.is_ok());
            assert_eq!(&*result.unwrap(), DECRYPTED_VAR_1);
        });
    }

    #[cfg(unix)]
    #[test]
    fn test_get_secure_var_paths_redact_not_unicode() {
        let dir = tempdir().unwrap();
        let key_file_path = dir.path().join("key-file");
        write_key_file(&key_file_path, VALID_KEY_FILE_CONTENTS);
        let key_file = key_file_path.to_str().unwrap();
        let name = "BAD_UNICODE_VAR";

        with_var(name, Some(invalid_secret_value()), || {
            let err = get_secure_var(name, key_file).unwrap_err();
            assert_notunicode_error_redacted(&err);

            let err = get_secure_var_or(name, key_file, "fallback").unwrap_err();
            assert_notunicode_error_redacted(&err);
        });
    }

    #[test]
    fn test_decrypt_known_ciphertexts() {
        let dir = tempdir().unwrap();
        let key_file_path = dir.path().join("key-file");
        write_key_file(&key_file_path, VALID_KEY_FILE_CONTENTS);
        let kf = key_file_path.to_str().unwrap();

        assert_eq!(&*decrypt(ENCRYPTED_VAR_1, kf).unwrap(), DECRYPTED_VAR_1);
        assert_eq!(&*decrypt(ENCRYPTED_VAR_2, kf).unwrap(), DECRYPTED_VAR_2);
    }

    #[test]
    fn test_decrypt_passthrough_short_values() {
        // Values shorter than 6 chars skip decryption regardless of content
        let dir = tempdir().unwrap();
        let key_file_path = dir.path().join("key-file");
        write_key_file(&key_file_path, VALID_KEY_FILE_CONTENTS);
        let kf = key_file_path.to_str().unwrap();

        assert_eq!(&*decrypt("", kf).unwrap(), "");
        assert_eq!(&*decrypt("short", kf).unwrap(), "short");
        assert_eq!(&*decrypt("12345", kf).unwrap(), "12345");
    }

    #[test]
    fn test_parse_key_file_reordered_fields() {
        let dir = tempdir().unwrap();
        let key_file_path = dir.path().join("key-file");
        // Write fields in iv, key, salt order (instead of salt, key, iv)
        write_key_file(
            &key_file_path,
            "iv=472A3557ADDD2525AD4E555738636A67\nkey=26D6EDD53A0AFA8FA1AA3FBCD2FFF2A0BF4809A4E04511F629FC732C2A42A8FC\nsalt=89A6A795C9CCECB5",
        );

        let (salt, key, iv) = parse_key_file(key_file_path.to_str().unwrap()).unwrap();
        assert_eq!(&*salt, "89A6A795C9CCECB5");
        assert_eq!(
            &*key,
            "26D6EDD53A0AFA8FA1AA3FBCD2FFF2A0BF4809A4E04511F629FC732C2A42A8FC"
        );
        assert_eq!(&*iv, "472A3557ADDD2525AD4E555738636A67");

        // Verify decryption still works with reordered key file
        assert_eq!(
            &*decrypt(ENCRYPTED_VAR_1, key_file_path.to_str().unwrap()).unwrap(),
            DECRYPTED_VAR_1
        );
    }

    #[test]
    fn test_parse_key_file_blank_lines_skipped() {
        let dir = tempdir().unwrap();
        let key_file_path = dir.path().join("key-file");
        {
            let mut file = File::create(&key_file_path).unwrap();
            writeln!(file).unwrap();
            writeln!(file, "salt=89A6A795C9CCECB5").unwrap();
            writeln!(file).unwrap();
            writeln!(
                file,
                "key=26D6EDD53A0AFA8FA1AA3FBCD2FFF2A0BF4809A4E04511F629FC732C2A42A8FC"
            )
            .unwrap();
            writeln!(file).unwrap();
            writeln!(file, "iv=472A3557ADDD2525AD4E555738636A67").unwrap();
            writeln!(file).unwrap();
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&key_file_path, std::fs::Permissions::from_mode(0o600))
                .unwrap();
        }

        let result = parse_key_file(key_file_path.to_str().unwrap());
        assert!(result.is_ok());
        let (salt, key, iv) = result.unwrap();
        assert_eq!(&*salt, "89A6A795C9CCECB5");
        assert_eq!(
            &*key,
            "26D6EDD53A0AFA8FA1AA3FBCD2FFF2A0BF4809A4E04511F629FC732C2A42A8FC"
        );
        assert_eq!(&*iv, "472A3557ADDD2525AD4E555738636A67");
    }

    #[test]
    fn test_get_secure_var_or_plain_text() {
        let dir = tempdir().unwrap();
        let key_file_path = dir.path().join("key-file");
        write_key_file(&key_file_path, VALID_KEY_FILE_CONTENTS);
        let kf = key_file_path.to_str().unwrap();

        with_var("PLAIN_SECURE_OR", Some("plain_value"), || {
            let result = get_secure_var_or("PLAIN_SECURE_OR", kf, "fallback");
            assert!(result.is_ok());
            assert_eq!(&*result.unwrap(), "plain_value");
        });
    }

    #[test]
    fn test_decrypt_missing_keyfile() {
        let missing_path = "/non/existent/keyfile";
        let result = decrypt(ENCRYPTED_VAR_1, missing_path);
        if let Err(EnvError::IoError(e)) = result {
            assert!(
                !e.to_string().contains(missing_path),
                "error must not leak path"
            );
            assert!(e.to_string().contains("cannot open key file"));
        } else {
            panic!("Expected IoError for missing key file");
        }
    }

    #[test]
    fn test_decrypt_invalid_hex() {
        let dir = tempdir().unwrap();
        let key_file_path = dir.path().join("key-file");
        write_key_file(&key_file_path, VALID_KEY_FILE_CONTENTS);
        let result = decrypt("+encs+ZZ", key_file_path.to_str().unwrap());
        assert!(matches!(result, Err(EnvError::DecryptionFailed(_))));
    }

    #[test]
    fn test_decrypt_empty_ciphertext() {
        let dir = tempdir().unwrap();
        let key_file_path = dir.path().join("key-file");
        write_key_file(&key_file_path, VALID_KEY_FILE_CONTENTS);
        let result = decrypt("+encs+", key_file_path.to_str().unwrap());
        let err = result.expect_err("expected error for empty ciphertext");
        assert!(matches!(err, EnvError::DecryptionFailed(_)));
        let msg = format!("{}", err);
        assert_eq!(msg, "decryption failed", "empty ciphertext must be opaque");
    }

    #[test]
    fn test_decrypt_opaque_errors() {
        let dir = tempdir().unwrap();
        let key_file_path = dir.path().join("key-file");
        write_key_file(&key_file_path, VALID_KEY_FILE_CONTENTS);
        // Invalid hex should produce an opaque error, not leak hex library details
        let result = decrypt("+encs+ZZ", key_file_path.to_str().unwrap());
        if let Err(EnvError::DecryptionFailed(ref inner)) = result {
            assert_eq!(
                inner, "decryption failed",
                "inner message must be opaque, got: {inner}"
            );
        } else {
            panic!("expected DecryptionFailed variant");
        }
        // Display must also be opaque
        let msg = format!("{}", result.unwrap_err());
        assert_eq!(
            msg, "decryption failed",
            "Display must be opaque, got: {msg}"
        );
    }

    #[test]
    fn test_parse_key_file_duplicate_key() {
        let dir = tempdir().unwrap();
        let key_file_path = dir.path().join("key-file");
        write_key_file(
            &key_file_path,
            "salt=89A6A795C9CCECB5\nkey=26D6EDD53A0AFA8FA1AA3FBCD2FFF2A0BF4809A4E04511F629FC732C2A42A8FC\nkey=AABBCCDDEEFF00112233445566778899AABBCCDDEEFF00112233445566778899\niv=472A3557ADDD2525AD4E555738636A67",
        );
        let result = parse_key_file(key_file_path.to_str().unwrap());
        let err = result.expect_err("expected error for duplicate key");
        assert_key_file_format_error(&err, "duplicate");
    }

    #[test]
    fn test_parse_key_file_oversize() {
        let dir = tempdir().unwrap();
        let key_file_path = dir.path().join("key-file");
        // Write a file > 1024 bytes (valid header + long salt value to pad)
        let long_salt = "x".repeat(1000);
        write_key_file(
            &key_file_path,
            &format!(
                "salt={long_salt}\nkey=26D6EDD53A0AFA8FA1AA3FBCD2FFF2A0BF4809A4E04511F629FC732C2A42A8FC\niv=472A3557ADDD2525AD4E555738636A67"
            ),
        );
        let result = parse_key_file(key_file_path.to_str().unwrap());
        let err = result.expect_err("expected error for oversize key file");
        assert_key_file_format_error(&err, "too large");
    }

    #[test]
    fn test_parse_key_file_no_equals_line() {
        let dir = tempdir().unwrap();
        let key_file_path = dir.path().join("key-file");
        write_key_file(
            &key_file_path,
            "salt=89A6A795C9CCECB5\nkey=26D6EDD53A0AFA8FA1AA3FBCD2FFF2A0BF4809A4E04511F629FC732C2A42A8FC\niv=472A3557ADDD2525AD4E555738636A67\ngarbage",
        );
        let result = parse_key_file(key_file_path.to_str().unwrap());
        let err = result.expect_err("expected error for line without '='");
        assert_key_file_format_error(&err, "invalid line");
    }

    #[cfg(unix)]
    #[test]
    fn test_parse_key_file_world_readable() {
        use std::os::unix::fs::PermissionsExt;

        let dir = tempdir().unwrap();
        let key_file_path = dir.path().join("key-file");
        write_key_file(&key_file_path, VALID_KEY_FILE_CONTENTS);
        std::fs::set_permissions(&key_file_path, std::fs::Permissions::from_mode(0o644)).unwrap();

        let result = parse_key_file(key_file_path.to_str().unwrap());
        let err = result.expect_err("expected error for world-readable key file");
        assert_key_file_format_error(&err, "world-readable");
    }

    #[cfg(unix)]
    #[test]
    fn test_parse_key_file_uses_open_file_descriptor_for_checks_and_read() {
        use std::os::unix::fs::PermissionsExt;

        let dir = tempdir().unwrap();
        let key_file_path = dir.path().join("key-file");
        let replacement_path = dir.path().join("replacement-key-file");
        write_key_file(&key_file_path, VALID_KEY_FILE_CONTENTS);
        write_key_file(
            &replacement_path,
            "salt=replacement\nkey=replacement\niv=replacement",
        );
        std::fs::set_permissions(&replacement_path, std::fs::Permissions::from_mode(0o644))
            .unwrap();

        let result = parse_key_file_with_opener(key_file_path.to_str().unwrap(), |path| {
            let file = File::open(path)?;
            std::fs::rename(&replacement_path, path)?;
            Ok(file)
        });

        let (salt, key, iv) = result.expect("expected parser to use the opened descriptor");
        assert_eq!(&*salt, "89A6A795C9CCECB5");
        assert_eq!(
            &*key,
            "26D6EDD53A0AFA8FA1AA3FBCD2FFF2A0BF4809A4E04511F629FC732C2A42A8FC"
        );
        assert_eq!(&*iv, "472A3557ADDD2525AD4E555738636A67");
    }

    #[test]
    fn test_env_error_formatting_redacts_crypto() {
        let decryption_err = EnvError::DecryptionFailed("secret stuff".to_string());
        let debug_str = format!("{:?}", decryption_err);
        assert!(
            debug_str.contains("[REDACTED]"),
            "Debug must redact inner message, got: {debug_str}"
        );
        assert!(
            !debug_str.contains("secret stuff"),
            "Debug must not expose inner message, got: {debug_str}"
        );

        let key_err = EnvError::KeyFileFormatError("secret stuff".to_string());
        let display_str = key_err.to_string();
        assert_eq!(display_str, "Key file format error");
        assert!(
            !display_str.contains("secret stuff"),
            "Display must not expose inner message, got: {display_str}"
        );

        let debug_str = format!("{:?}", key_err);
        assert!(
            debug_str.contains("[REDACTED]"),
            "Debug must redact inner message, got: {debug_str}"
        );
        assert!(
            !debug_str.contains("secret stuff"),
            "Debug must not expose inner message, got: {debug_str}"
        );
    }

    #[test]
    fn test_parse_key_file_no_path_leak() {
        let nonexistent = "/tmp/nonexistent-key-file-xyz-abc";
        let result = parse_key_file(nonexistent);
        let err = result.expect_err("expected IoError for nonexistent path");
        let msg = format!("{}", err);
        assert!(
            !msg.contains(nonexistent),
            "error must not leak file path, got: {msg}"
        );
    }
}
