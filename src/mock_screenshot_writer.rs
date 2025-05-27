use screenshots::Image;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use crate::{ScreenshotWriter, ScreenshotError};

/// A mock implementation of ScreenshotWriter for testing purposes.
pub struct MockScreenshotWriter {
    path: PathBuf,
    call_count: Arc<AtomicUsize>,
    black_image: Image,
    white_image: Image,
    mixed_image: Image,
}

impl MockScreenshotWriter {
    /// Creates a new MockScreenshotWriter with the given path.
    pub fn new(path: PathBuf) -> Self {
        // Create a 2x2 black image (all pixels black)
        let black_pixels = vec![
            0, 0, 0, 255, 0, 0, 0, 255,
            0, 0, 0, 255, 0, 0, 0, 255
        ];
        let black_image = Image::new(2, 2, black_pixels);
        
        // Create a 2x2 white image (all pixels white)
        let white_pixels = vec![
            255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255
        ];
        let white_image = Image::new(2, 2, white_pixels);
        
        // Create a 2x2 mixed image (black, white, black, red)
        let mixed_pixels = vec![
            0, 0, 0, 255,       // Black
            255, 255, 255, 255, // White
            0, 0, 0, 255,       // Black
            255, 0, 0, 255      // Red
        ];
        let mixed_image = Image::new(2, 2, mixed_pixels);
        
        Self {
            path,
            call_count: Arc::new(AtomicUsize::new(0)),
            black_image,
            white_image,
            mixed_image,
        }
    }

    /// Helper method to clone an image
    fn get_image(&self, image: &Image) -> Image {
        let buffer = image.buffer().to_vec();
        Image::new(image.width(), image.height(), buffer)
    }
    
    /// Internal method to check if an image is empty (all black or all white)
    fn is_image_empty_internal(&self, image: &Image) -> bool {
        let width = image.width() as usize;
        let height = image.height() as usize;
        let buffer = image.buffer();
        
        if width == 0 || height == 0 {
            return true;
        }
        
        // Check the first pixel to determine if we're checking for all black or all white
        if buffer.len() < 4 {
            return true;
        }
        
        let first_pixel = &buffer[0..4];
        let is_black = first_pixel[0] == 0 && first_pixel[1] == 0 && first_pixel[2] == 0;
        let is_white = first_pixel[0] == 255 && first_pixel[1] == 255 && first_pixel[2] == 255;
        
        if !is_black && !is_white {
            return false;
        }
        
        // Check all other pixels match the first one
        for i in 0..(width * height) {
            let idx = i * 4;
            if idx + 3 >= buffer.len() {
                continue;
            }
            
            let r = buffer[idx];
            let g = buffer[idx + 1];
            let b = buffer[idx + 2];
            
            if is_black && (r != 0 || g != 0 || b != 0) {
                return false;
            }
            if is_white && (r != 255 || g != 255 || b != 255) {
                return false;
            }
        }
        
        true // All pixels match the first one (all black or all white)
    }
}

impl ScreenshotWriter for MockScreenshotWriter {
    fn new(path: PathBuf) -> Self {
        MockScreenshotWriter::new(path)
    }
    
    fn capture_screen(&self) -> Result<Image, ScreenshotError> {
        let call_count = self.call_count.fetch_add(1, Ordering::SeqCst);
        match call_count {
            0 => Ok(self.get_image(&self.black_image)),
            1 => Ok(self.get_image(&self.white_image)),
            _ => Ok(self.get_image(&self.mixed_image)),
        }
    }
    
    fn is_image_empty(&self, image: &Image) -> bool {
        self.is_image_empty_internal(image)
    }
    
    fn date_folder_path(&self) -> PathBuf {
        self.path.clone()
    }
    
    fn write_screenshot(&mut self) -> Result<(), ScreenshotError> {
        // No-op for testing
        Ok(())
    }
}
