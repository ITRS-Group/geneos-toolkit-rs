use std::env;
use std::error::Error;
use std::fmt;

#[non_exhaustive]
pub enum EnvError {
    VarError(env::VarError),
    IoError(std::io::Error),
    MissingSecureEnvSupport,
    #[cfg(feature = "secure-env")]
    DecryptionFailed(String),
    #[cfg(feature = "secure-env")]
    MissingKeyFile,
    #[cfg(feature = "secure-env")]
    KeyFileFormatError(String),
}

impl fmt::Debug for EnvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnvError::VarError(env::VarError::NotUnicode(_)) => {
                write!(f, "VarError(NotUnicode([REDACTED]))")
            }
            EnvError::VarError(e) => f.debug_tuple("VarError").field(e).finish(),
            EnvError::IoError(e) => f.debug_tuple("IoError").field(e).finish(),
            EnvError::MissingSecureEnvSupport => write!(f, "MissingSecureEnvSupport"),
            #[cfg(feature = "secure-env")]
            EnvError::DecryptionFailed(_) => write!(f, "DecryptionFailed([REDACTED])"),
            #[cfg(feature = "secure-env")]
            EnvError::MissingKeyFile => write!(f, "MissingKeyFile"),
            #[cfg(feature = "secure-env")]
            EnvError::KeyFileFormatError(_) => write!(f, "KeyFileFormatError([REDACTED])"),
        }
    }
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

impl fmt::Display for EnvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnvError::VarError(env::VarError::NotUnicode(_)) => write!(
                f,
                "Environment variable error: environment variable value is not valid Unicode"
            ),
            EnvError::VarError(e) => write!(f, "Environment variable error: {}", e),
            EnvError::IoError(e) => write!(f, "IO error: {}", e),
            EnvError::MissingSecureEnvSupport => {
                write!(
                    f,
                    "Secure environment support is disabled (enable the 'secure-env' feature)"
                )
            }
            #[cfg(feature = "secure-env")]
            EnvError::DecryptionFailed(_) => write!(f, "decryption failed"),
            #[cfg(feature = "secure-env")]
            EnvError::MissingKeyFile => write!(f, "Missing key file for decryption"),
            #[cfg(feature = "secure-env")]
            EnvError::KeyFileFormatError(_) => write!(f, "Key file format error"),
        }
    }
}

impl Error for EnvError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            EnvError::VarError(env::VarError::NotUnicode(_)) => None,
            EnvError::VarError(e) => Some(e),
            EnvError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

/// Retrieves an environment variable's value.
/// Returns `MissingSecureEnvSupport` if the value is encrypted and `secure-env` is disabled.
///
/// # Example (ignored to avoid mutating process env in doctest)
/// ```ignore
/// use geneos_toolkit::env::get_var;
/// std::env::set_var("PLAIN_EXAMPLE", "ok");
/// assert_eq!(get_var("PLAIN_EXAMPLE").unwrap(), "ok");
/// ```
pub fn get_var(name: &str) -> Result<String, EnvError> {
    let val = env::var(name)?;
    #[cfg(not(feature = "secure-env"))]
    if is_encrypted(&val) {
        return Err(EnvError::MissingSecureEnvSupport);
    }
    Ok(val)
}

/// Retrieves an environment variable's value or returns a default if not set.
/// Returns an error if the value is encrypted and secure support is disabled.
///
/// # Example (ignored to avoid mutating process env in doctest)
/// ```ignore
/// use geneos_toolkit::env::get_var_or;
/// assert_eq!(get_var_or("MISSING", "fallback").unwrap(), "fallback");
/// ```
pub fn get_var_or(name: &str, default: &str) -> Result<String, EnvError> {
    match env::var(name) {
        Ok(val) => {
            #[cfg(not(feature = "secure-env"))]
            if is_encrypted(&val) {
                return Err(EnvError::MissingSecureEnvSupport);
            }
            Ok(val)
        }
        Err(env::VarError::NotPresent) => Ok(default.to_string()),
        Err(e) => Err(EnvError::VarError(e)),
    }
}

/// Checks if a string slice is encrypted. Encrypted values start with "+encs+".
pub fn is_encrypted(value: &str) -> bool {
    value.starts_with("+encs+")
}

#[cfg(test)]
mod tests {
    use super::*;
    use temp_env::with_var;

    #[cfg(unix)]
    fn invalid_secret_value() -> std::ffi::OsString {
        use std::os::unix::ffi::OsStringExt;

        std::ffi::OsString::from_vec(b"secret-\xFF-token".to_vec())
    }

    #[cfg(unix)]
    fn assert_notunicode_error_redacted(err: &EnvError) {
        assert!(
            matches!(err, EnvError::VarError(env::VarError::NotUnicode(_))),
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
        assert!(!formatted.contains("secret"));
        assert!(!formatted.contains("token"));
    }

    #[test]
    fn test_get_env() {
        with_var("TEST_VAR", Some("test_value"), || {
            assert_eq!(get_var("TEST_VAR").unwrap(), "test_value");
            assert_eq!(get_var_or("TEST_VAR", "default").unwrap(), "test_value");
        });

        with_var::<_, &str, _, _>("NON_EXISTENT_VAR", None, || {
            assert!(get_var("NON_EXISTENT_VAR").is_err());
            assert_eq!(
                get_var_or("NON_EXISTENT_VAR", "default").unwrap(),
                "default"
            );
        });
    }

    #[cfg(unix)]
    #[test]
    fn test_notunicode_var_error_redacts_display_debug_and_source() {
        let err = EnvError::VarError(env::VarError::NotUnicode(invalid_secret_value()));
        assert_notunicode_error_redacted(&err);
    }

    #[cfg(unix)]
    #[test]
    fn test_get_var_redacts_notunicode_errors() {
        let name = "GENEOS_TOOLKIT_TEST_INVALID_UNICODE";

        with_var(name, Some(invalid_secret_value()), || {
            let err = get_var(name).unwrap_err();
            assert_notunicode_error_redacted(&err);

            let err = get_var_or(name, "default").unwrap_err();
            assert_notunicode_error_redacted(&err);
        });
    }

    #[test]
    fn test_is_encrypted() {
        assert!(is_encrypted("+encs+1234567890ABCDEF"));
        assert!(!is_encrypted("plain_text"));
        assert!(!is_encrypted(""));
    }

    #[test]
    fn test_is_encrypted_edge_cases() {
        // Bare prefix with no payload is still "encrypted"
        assert!(is_encrypted("+encs+"));

        // Incomplete prefix variants are not encrypted
        assert!(!is_encrypted("+encs"));
        assert!(!is_encrypted("+enc+"));
        assert!(!is_encrypted("+ENCS+1234"));
        assert!(!is_encrypted("encs+1234"));
        assert!(!is_encrypted(" +encs+1234"));
    }
}
