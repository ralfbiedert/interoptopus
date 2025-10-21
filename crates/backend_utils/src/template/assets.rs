use crate::Error;
use std::path::Path;

pub struct Assets {}

impl Assets {
    pub fn load(asset_file: impl AsRef<Path>) -> Result<Self, Error> {
        Ok(Self {})
    }
}

pub fn pack_assets(asset_file: impl AsRef<Path>, root: impl AsRef<Path>) -> Result<(), Error> {
    Ok(())
}
