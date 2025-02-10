use anyhow::{anyhow, Context, Result};
use image::{GenericImageView, ImageBuffer, Rgba};
use std::ffi::OsString;
use std::path::PathBuf;

const ASSETS_DIR: &str = "assets";

pub type AssetsPath = PathBuf;

pub struct LoadedImage {
    pub data: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub height: u32,
    pub width: u32,
    pub file_name: String,
    pub file_type: OsString,
}

impl LoadedImage {
    pub fn from_path(asset_path: &AssetsPath, file_name: &str) -> Result<LoadedImage> {
        let full = asset_path.join(file_name);
        let img = image::ImageReader::open(&full)?.decode()?;
        let size = img.dimensions();
        let rgba_img = img.into_rgba8();
        let file_type = full
            .extension()
            .map(|s| s.to_os_string())
            .ok_or(anyhow!("file extension missing"))?;

        Ok(LoadedImage {
            data: rgba_img,
            width: size.0,
            height: size.1,
            file_type,
            file_name: file_name.to_string(),
        })
    }
}

pub fn make_assets_path() -> Result<AssetsPath> {
    let current_dir = std::env::current_dir().context("getting current directory")?;
    Ok(current_dir.join(ASSETS_DIR))
}
