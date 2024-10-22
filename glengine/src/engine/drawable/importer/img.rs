use image;
use image::GenericImageView;

pub struct Image{
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl Image {
    pub fn load(path: &str) -> Result<Self, String> {
        let img = image::open(path).map_err(|e| e.to_string())?;
        let (width, height) = img.dimensions();
        let data = img.into_rgba8().into_raw();
        Ok(Self { width, height, data })
    }
}