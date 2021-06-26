use crate::Config;
use interoptopus::{Error, Library};
use std::fs::{canonicalize, create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Since Rust produces `debug` and `release` assemblies (DLLs), select which one.
pub enum Assembly {
    /// Always pick the debug DLL.
    Debug,
    /// Always pick the release DLL.
    Release,
    /// Pick the DLL most recently produced.
    MostRecent,
}

pub struct UnityReloadHelper {
    pub inventory: Library,
    pub config: Config,
    /// We'd love to get this automatically ... should point to your used `target/` folder.
    pub target_path_hint: String,
    /// Type of assembly to copy.
    pub assembly: Assembly,
    /// Asset name to produce in Unity (e.g., `MyLibraries/MyRustcode`)
    pub asset_name: String,
    /// Interop files related to this assembly also needing an update.
    pub interop_files: Vec<String>,
}

fn get_platform_dll_name(dll: &str) -> String {
    format!("{}.dll", dll)
}

fn find_assembly_for_current_platform(root: &str, config: &Config, assembly: &Assembly) -> Option<String> {
    // TODO: We should get this from `build.rs`, but that requires more setup
    // work on the user's side, so we cheat a bit here and hope
    let root = PathBuf::from(root);
    let root = match assembly {
        Assembly::Debug => root.join("debug"),
        Assembly::Release => root.join("release"),
        Assembly::MostRecent => todo!("Not implemented yet"),
    };

    // TODO: Don't just handle windows
    let root = root.join(get_platform_dll_name(&config.dll_name));

    Some(canonicalize(root).ok()?.to_str()?.to_string())
}

impl UnityReloadHelper {
    pub fn write_to_asset_folder<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let template = include_str!("InteroptopusHotReload.cs.template");

        let dll_file = find_assembly_for_current_platform(&self.target_path_hint, &self.config, &self.assembly).ok_or(Error::FileNotFound)?;
        let dll_name = get_platform_dll_name(&self.config.dll_name);

        let mut lines = Vec::new();

        for file in &self.interop_files {
            let full_path = canonicalize(PathBuf::from(file))?.to_str().ok_or(Error::FileNotFound)?.to_string();
            lines.push(format!(r#"@"{}""#, full_path));
        }

        let to_write = template;
        let to_write = to_write.replace("%ASSET_NAME%", &self.asset_name);
        let to_write = to_write.replace("%DLL_SOURCE%", &dll_file);
        let to_write = to_write.replace("%DLL_NAME%", &dll_name);
        let to_write = to_write.replace("%INTEROP_FILES%", &lines.join(", "));
        // let to_write = to_write.replace("%INTEROP_SOURCE%", &self.interop_source);

        // Create output folder
        let asset_path = path.as_ref().join(&self.asset_name);
        let helper_file = asset_path.join(format!("Interoptopus.HotReload.{}.cs", self.asset_name));

        create_dir_all(&asset_path)?;

        let mut file = File::create(helper_file)?;

        writeln!(file, "{}", to_write)?;

        Ok(())
    }
}
