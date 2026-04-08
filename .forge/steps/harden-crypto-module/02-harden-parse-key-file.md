# Step 02: Harden parse_key_file

## Objective
Add input validation to `parse_key_file`: file size limit, Unix permission check, duplicate key detection, reject malformed lines, sanitize error messages.

## Deliverables
- `src/secure_env.rs`

## Depends On
None

## Steps

1. Before opening the file, check size via `fs::metadata(path)`:
   ```rust
   let meta = fs::metadata(path)
       .map_err(|_| EnvError::IoError(io::Error::new(io::ErrorKind::NotFound, "cannot open key file")))?;
   if meta.len() > 1024 {
       return Err(EnvError::KeyFileFormatError("key file too large".to_string()));
   }
   ```

2. Add `#[cfg(unix)]` permission check after metadata:
   ```rust
   #[cfg(unix)]
   {
       use std::os::unix::fs::PermissionsExt;
       if meta.permissions().mode() & 0o004 != 0 {
           return Err(EnvError::KeyFileFormatError("key file is world-readable".to_string()));
       }
   }
   ```

3. Change `File::open` error to omit the path:
   ```rust
   let file = File::open(path)
       .map_err(|_| EnvError::IoError(io::Error::new(io::ErrorKind::NotFound, "cannot open key file")))?;
   ```

4. Change `None` arm (no `=`) from silent skip to error:
   ```rust
   None => {
       return Err(EnvError::KeyFileFormatError(
           format!("invalid line {} in key file", line_num)
       ));
   }
   ```

5. Change `Some((other, _))` arm to not echo the key name:
   ```rust
   Some((_, _)) => {
       return Err(EnvError::KeyFileFormatError(
           format!("unexpected key at line {} in key file", line_num)
       ));
   }
   ```

6. Add duplicate key detection — check if `salt`/`key`/`iv` is already `Some` before assigning:
   ```rust
   Some(("salt", value)) => {
       if salt.is_some() {
           return Err(EnvError::KeyFileFormatError("duplicate salt in key file".to_string()));
       }
       salt = Some(value.to_string());
   }
   ```
   (Repeat for key and iv.)

7. Sanitize the missing-field error messages to not reference internal field names:
   ```rust
   let salt = salt.ok_or_else(|| EnvError::KeyFileFormatError("incomplete key file".to_string()))?;
   ```
   (Same message for all three fields, or distinct but generic.)

## Acceptance Criteria
- AC2: Error messages do not echo file paths or file content
- AC3: Duplicate keys rejected
- AC4: Files > 1 KiB rejected
- AC5: Lines without `=` rejected
- AC6: World-readable files rejected on Unix
