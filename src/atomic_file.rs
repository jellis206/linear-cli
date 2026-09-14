//! Atomic, private (0600 on Unix) file writes: write to a randomized sibling
//! temp path with `create_new` (fails instead of following a pre-existing
//! symlink), then rename into place. Shared by every call site that writes
//! secrets or user files to disk (config, cache, update state, uploads, export).

use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

#[cfg(unix)]
fn create_private_file(path: &Path) -> Result<fs::File> {
    use std::os::unix::fs::OpenOptionsExt;

    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)
        .with_context(|| format!("Failed to create file: {}", path.display()))
}

#[cfg(not(unix))]
fn create_private_file(path: &Path) -> Result<fs::File> {
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .with_context(|| format!("Failed to create file: {}", path.display()))
}

fn atomic_temp_path(path: &Path) -> Result<PathBuf> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .context("Output path must include a file name")?;
    let unique = format!(
        ".{}.tmp-{}-{}",
        file_name,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_nanos()
    );
    Ok(parent.join(unique))
}

/// A file opened at a randomized temp path that is renamed into place on
/// `commit()`, or deleted if dropped without committing.
pub struct AtomicPrivateFile {
    file: Option<fs::File>,
    temp_path: PathBuf,
    final_path: PathBuf,
    committed: bool,
}

impl AtomicPrivateFile {
    pub fn create(path: &Path) -> Result<Self> {
        let temp_path = atomic_temp_path(path)?;
        let file = create_private_file(&temp_path)
            .with_context(|| format!("Failed to create file: {}", temp_path.display()))?;
        Ok(Self {
            file: Some(file),
            temp_path,
            final_path: path.to_path_buf(),
            committed: false,
        })
    }

    pub fn commit(&mut self) -> Result<()> {
        if self.committed {
            return Ok(());
        }

        if let Some(mut file) = self.file.take() {
            file.flush()?;
            file.sync_all()?;
            drop(file);
        }

        // On Windows, rename fails if the destination exists.
        #[cfg(windows)]
        {
            let _ = fs::remove_file(&self.final_path);
        }
        fs::rename(&self.temp_path, &self.final_path)
            .with_context(|| format!("Failed to atomically write {}", self.final_path.display()))?;
        self.committed = true;
        Ok(())
    }

    #[cfg(test)]
    pub fn file_mut(&mut self) -> &mut fs::File {
        self.file.as_mut().expect("atomic file should be open")
    }
}

impl Write for AtomicPrivateFile {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.file
            .as_mut()
            .expect("atomic file should be open")
            .write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.file
            .as_mut()
            .expect("atomic file should be open")
            .flush()
    }
}

impl Drop for AtomicPrivateFile {
    fn drop(&mut self) {
        if !self.committed {
            let _ = self.file.take();
            let _ = fs::remove_file(&self.temp_path);
        }
    }
}

/// Write `contents` to `path` atomically and privately in one call — the
/// common case for small JSON/TOML state files (config, cache, update state).
pub fn write_private(path: &Path, contents: &[u8]) -> Result<()> {
    let mut file = AtomicPrivateFile::create(path)?;
    file.write_all(contents)?;
    file.commit()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_private_creates_file_with_contents() {
        let dir = std::env::temp_dir().join(format!("atomic-file-test-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("secret.toml");

        write_private(&path, b"hello").unwrap();
        assert_eq!(fs::read(&path).unwrap(), b"hello");

        // Overwriting an existing file must still succeed (rename replaces it).
        write_private(&path, b"world").unwrap();
        assert_eq!(fs::read(&path).unwrap(), b"world");

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = fs::metadata(&path).unwrap().permissions().mode() & 0o777;
            assert_eq!(mode, 0o600);
        }

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn refuses_to_follow_a_pre_existing_symlink_at_the_temp_path() {
        #[cfg(unix)]
        {
            let dir = std::env::temp_dir()
                .join(format!("atomic-file-symlink-test-{}", std::process::id()));
            fs::create_dir_all(&dir).unwrap();
            let target = dir.join("config.toml");
            let attacker_target = dir.join("attacker-owned-file");

            let temp_path = atomic_temp_path(&target).unwrap();
            std::os::unix::fs::symlink(&attacker_target, &temp_path).unwrap();

            // create_new must fail because the temp path already exists (as a symlink),
            // instead of opening/truncating through it.
            let result = create_private_file(&temp_path);
            assert!(result.is_err());
            assert!(!attacker_target.exists());

            let _ = fs::remove_dir_all(&dir);
        }
    }
}
