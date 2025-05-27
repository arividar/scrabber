use screenshots::Image;
use std::path::PathBuf;
use image;

#[derive(Debug)]
pub enum ScreenshotError {
    IoError(std::io::Error),
    ImageError(image::ImageError),
    Other(String),
}

impl From<std::io::Error> for ScreenshotError {
    fn from(err: std::io::Error) -> Self {
        ScreenshotError::IoError(err)
    }
}

impl From<image::ImageError> for ScreenshotError {
    fn from(err: image::ImageError) -> Self {
        ScreenshotError::ImageError(err)
    }
}

impl std::fmt::Display for ScreenshotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScreenshotError::IoError(e) => write!(f, "IO error: {}", e),
            ScreenshotError::ImageError(e) => write!(f, "Image error: {}", e),
            ScreenshotError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for ScreenshotError {}

pub trait ScreenshotWriter: Send + Sync {
    fn new(path: PathBuf) -> Self where Self: Sized;
    fn capture_screen(&self) -> Result<Image, ScreenshotError>;
    fn is_image_empty(&self, image: &Image) -> bool;
    fn date_folder_path(&self) -> PathBuf;
    fn write_screenshot(&mut self) -> Result<(), ScreenshotError>;
}
