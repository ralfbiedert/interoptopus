use crate::Error;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Runtime asset loader that can load files from an embedded tar archive
pub struct Assets {
    files: HashMap<String, Vec<u8>>,
}

impl Assets {
    /// Internal method to load from any Read source
    pub fn from_reader(reader: impl Read) -> Result<Self, Error> {
        let mut archive = tar::Archive::new(reader);
        let mut files = HashMap::new();

        for entry in archive.entries()? {
            let mut entry = entry?;
            let path = entry.path()?;
            let path_str = path.to_string_lossy().to_string();

            let mut contents = Vec::new();
            entry.read_to_end(&mut contents)?;

            files.insert(path_str, contents);
        }

        Ok(Self { files })
    }

    /// Load a file as a UTF-8 string
    pub fn get_string(&self, path: impl AsRef<str>) -> Result<String, Error> {
        let path = path.as_ref();
        let bytes = self.files.get(path).ok_or_else(|| Error::AssetNotFound(path.to_string()))?;
        String::from_utf8(bytes.clone()).map_err(|e| Error::AssetUtf8Error(path.to_string(), e))
    }

    /// Load a file as raw bytes
    pub fn get_bytes(&self, path: impl AsRef<str>) -> Result<&[u8], Error> {
        let path = path.as_ref();
        self.files.get(path).map(|v| v.as_slice()).ok_or_else(|| Error::AssetNotFound(path.to_string()))
    }

    /// Check if a file exists in the assets
    pub fn contains(&self, path: impl AsRef<str>) -> bool {
        self.files.contains_key(path.as_ref())
    }

    /// List all asset paths
    pub fn list(&self) -> impl Iterator<Item = &str> {
        self.files.keys().map(|s| s.as_str())
    }
}

/// Pack all files under `root` into a tar archive at `out_path`
///
/// This function is meant to be called from build.rs scripts. It will:
/// 1. Walk all files under the `root` directory
/// 2. Create a tar archive containing all files with paths relative to `root`
/// 3. Write the archive to `out_path`
///
/// # Example
///
/// ```no_run
/// // In build.rs
/// use interoptopus_backends::template::pack_assets;
/// use std::path::PathBuf;
///
/// fn main() {
///     let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("templates.tar");
///     pack_assets(&out_path, "templates/").unwrap();
///     println!("cargo:rerun-if-changed=templates/");
///     println!("cargo:warning=Packed assets to {}", out_path.display());
/// }
/// ```
pub fn pack_assets(out_path: impl AsRef<Path>, root: impl AsRef<Path>) -> Result<(), Error> {
    let root = root.as_ref();
    let out_path = out_path.as_ref();

    let tar_file = File::create(out_path)?;
    let mut builder = tar::Builder::new(tar_file);

    // Walk the directory tree
    walk_dir(root, root, &mut builder)?;

    builder.finish()?;

    Ok(())
}

/// Recursively walk a directory and add all files to the tar archive
fn walk_dir(root: &Path, current: &Path, builder: &mut tar::Builder<File>) -> Result<(), Error> {
    if !current.is_dir() {
        return Ok(());
    }

    for entry in std::fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            walk_dir(root, &path, builder)?;
        } else if path.is_file() {
            let relative = path.strip_prefix(root)?;
            builder.append_path_with_name(&path, relative)?;
        }
    }

    Ok(())
}
