# Step 03: Redact Debug + Salt Comment

## Objective
Replace `#[derive(Debug)]` on `EnvError` with a manual `Debug` impl that redacts crypto-sensitive variants. Add explanatory comment for the intentionally-unused salt field.

## Deliverables
- `src/env.rs`
- `src/secure_env.rs`

## Depends On
None

## Steps

1. In `src/env.rs`, remove `#[derive(Debug)]` from `EnvError`.

2. Add manual `fmt::Debug` impl:
   ```rust
   impl fmt::Debug for EnvError {
       fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
           match self {
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
   ```

3. In `src/secure_env.rs` line 62, add a comment explaining the discarded salt:
   ```rust
   // Salt was consumed during PBKDF key derivation by Geneos Gateway;
   // only key and IV are needed for decryption.
   let (_, key_hex, iv_hex) = parse_key_file(key_file)?;
   ```

## Acceptance Criteria
- AC9: Salt has explanatory comment
- AC10: Debug output redacts DecryptionFailed and KeyFileFormatError inner strings
