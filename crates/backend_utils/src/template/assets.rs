use crate::Error;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

/// Runtime asset loader that can load files from an embedded tar archive
pub struct Assets {
    files: HashMap<String, Vec<u8>>,
}

impl Assets {
    /// Internal method to load from any Read source
    pub fn from_reader<R: Read>(reader: R) -> Result<Self, Error> {
        let mut archive = tar::Archive::new(reader);
        let mut files = HashMap::new();

        for entry in archive.entries().map_err(Error::AssetError)? {
            let mut entry = entry.map_err(Error::AssetError)?;
            let path = entry.path().map_err(Error::AssetError)?;
            let path_str = path.to_string_lossy().to_string();

            let mut contents = Vec::new();
            entry.read_to_end(&mut contents).map_err(Error::AssetError)?;

            files.insert(path_str, contents);
        }

        Ok(Self { files })
    }

    /// Load a file as a UTF-8 string
    pub fn load_string(&self, path: impl AsRef<str>) -> Result<String, Error> {
        let path = path.as_ref();
        let bytes = self.files.get(path).ok_or_else(|| Error::AssetNotFound(path.to_string()))?;
        String::from_utf8(bytes.clone()).map_err(|e| Error::AssetUtf8Error(path.to_string(), e))
    }

    /// Load a file as raw bytes
    pub fn load_bytes(&self, path: impl AsRef<str>) -> Result<&[u8], Error> {
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

/// Pack all files under `root` into a tar archive at `asset_file`
///
/// This function is meant to be called from build.rs scripts. It will:
/// 1. Walk all files under the `root` directory
/// 2. Create a tar archive containing all files with paths relative to `root`
/// 3. Write the archive to `asset_file`
///
/// The output file will be placed in `OUT_DIR` so it can be included with `include_bytes!`
///
/// # Example
///
/// ```no_run
/// // In build.rs
/// use interoptopus_backends::template::pack_assets;
///
/// fn main() {
///     pack_assets("templates.tar", "templates/").unwrap();
/// }
/// ```
pub fn pack_assets(asset_file: impl AsRef<Path>, root: impl AsRef<Path>) -> Result<(), Error> {
    let root = root.as_ref();
    let out_dir = std::env::var("OUT_DIR").map_err(|_| Error::MissingOutDir)?;
    let out_path = PathBuf::from(out_dir).join(asset_file.as_ref());

    let tar_file = File::create(&out_path).map_err(Error::AssetError)?;
    let mut builder = tar::Builder::new(tar_file);

    // Walk the directory tree
    walk_dir(root, root, &mut builder)?;

    builder.finish().map_err(Error::AssetError)?;

    println!("cargo:rerun-if-changed={}", root.display());
    println!("cargo:warning=Packed assets to {}", out_path.display());

    Ok(())
}

/// Recursively walk a directory and add all files to the tar archive
fn walk_dir(root: &Path, current: &Path, builder: &mut tar::Builder<File>) -> Result<(), Error> {
    if !current.is_dir() {
        return Ok(());
    }

    for entry in std::fs::read_dir(current).map_err(Error::AssetError)? {
        let entry = entry.map_err(Error::AssetError)?;
        let path = entry.path();

        if path.is_dir() {
            walk_dir(root, &path, builder)?;
        } else if path.is_file() {
            let relative = path.strip_prefix(root).map_err(|_| Error::PathStripError)?;
            builder.append_path_with_name(&path, relative).map_err(Error::AssetError)?;
        }
    }

    Ok(())
}
