use anyhow::{anyhow, Result};
use image::{GenericImageView, ImageBuffer, Rgba};
use std::path::PathBuf;

pub struct LoadedImage {
    pub data: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub height: u32,
    pub width: u32,
    pub file_name: String,
}

impl LoadedImage {
    pub fn from_path(path: PathBuf) -> Result<LoadedImage> {
        let img = image::ImageReader::open(&path)?.decode()?;
        let size = img.dimensions();
        let rgba_img = img.into_rgba8();
        let file_name = path
            .file_stem()
            .ok_or(anyhow!("expected file name"))?
            .to_str()
            .ok_or(anyhow!("file path not a string"))?
            .to_string();

        Ok(LoadedImage {
            data: rgba_img,
            height: size.0,
            width: size.1,
            file_name,
        })
    }
}
