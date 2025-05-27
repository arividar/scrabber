use image::{ImageBuffer, Rgba};
use screenshots::Image;
use std::path::PathBuf;

// Helper function to create a test image from RGBA pixels
fn create_test_image(width: u32, height: u32, pixels: &[u8]) -> Image {
    Image::new(width, height, pixels.to_vec())
}

// Helper function to convert ImageBuffer to screenshots::Image
fn image_buffer_to_screenshot(img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Image {
    let (width, height) = img.dimensions();
    let mut pixels = Vec::with_capacity((width * height * 4) as usize);
    
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            pixels.push(pixel[0]);
            pixels.push(pixel[1]);
            pixels.push(pixel[2]);
            pixels.push(pixel[3]);
        }
    }
    
    Image::new(width, height, pixels)
}

#[test]
fn test_is_image_empty() {
    use super::is_image_empty;
    use image::{ImageBuffer, Rgba};
    
    // Test all black
    let mut img = ImageBuffer::new(2, 2);
    for y in 0..2 {
        for x in 0..2 {
            img.put_pixel(x, y, Rgba([0, 0, 0, 255]));
        }
    }
    let screenshot = image_buffer_to_screenshot(&img);
    assert!(is_image_empty(&screenshot), "All black image should be empty");
    
    // Test all white
    let mut img = ImageBuffer::new(2, 2);
    for y in 0..2 {
        for x in 0..2 {
            img.put_pixel(x, y, Rgba([255, 255, 255, 255]));
        }
    }
    let screenshot = image_buffer_to_screenshot(&img);
    assert!(is_image_empty(&screenshot), "All white image should be empty");
    
    // Test mixed black and white
    let mut img = ImageBuffer::new(2, 2);
    img.put_pixel(0, 0, Rgba([0, 0, 0, 255]));
    img.put_pixel(1, 0, Rgba([255, 255, 255, 255]));
    img.put_pixel(0, 1, Rgba([0, 0, 0, 255]));
    img.put_pixel(1, 1, Rgba([0, 0, 0, 255]));
    let screenshot = image_buffer_to_screenshot(&img);
    assert!(!is_image_empty(&screenshot), "Mixed black and white image should not be empty");
    
    // Test with a red pixel
    let mut img = ImageBuffer::new(2, 2);
    img.put_pixel(0, 0, Rgba([0, 0, 0, 255]));
    img.put_pixel(1, 0, Rgba([255, 0, 0, 255]));
    img.put_pixel(0, 1, Rgba([0, 0, 0, 255]));
    img.put_pixel(1, 1, Rgba([0, 0, 0, 255]));
    let screenshot = image_buffer_to_screenshot(&img);
    assert!(!is_image_empty(&screenshot), "Image with red pixel should not be empty");
    
    // Test empty image
    let empty_img = Image::new(0, 0, vec![]);
    assert!(is_image_empty(&empty_img), "Empty image should be considered empty");
}

#[test]
fn test_screenshot_writer_trait() {
    use super::{ScreenshotWriter, ScreenshotError};
    
    // Create a test implementation of ScreenshotWriter
    struct TestScreenshotWriter {
        call_count: std::sync::atomic::AtomicUsize,
    }
    
    impl TestScreenshotWriter {
        fn new() -> Self {
            Self {
                call_count: std::sync::atomic::AtomicUsize::new(0),
            }
        }
    }
    
    impl ScreenshotWriter for TestScreenshotWriter {
        fn new(_path: PathBuf) -> Self { 
            TestScreenshotWriter::new()
        }
        
        fn capture_screen(&self) -> Result<Image, ScreenshotError> { 
            let count = self.call_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            let (width, height, pixels) = match count {
                0 => (2, 2, vec![0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255]), // All black
                1 => (2, 2, vec![255, 255, 255, 255; 16]), // All white
                _ => (2, 2, vec![0, 0, 0, 255, 255, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255]), // Black with one red pixel
            };
            Ok(Image::new(width, height, pixels))
        }
        
        fn is_image_empty(&self, image: &Image) -> bool {
            // For testing, just forward to the actual implementation
            super::is_image_empty(image)
        }
        
        fn date_folder_path(&self) -> PathBuf {
            PathBuf::from("/tmp")
        }
        
        fn write_screenshot(&mut self) -> Result<(), ScreenshotError> {
            let _ = self.capture_screen()?;
            Ok(())
        }
    }
    
    let mut ssw = TestScreenshotWriter::new();
    
    // First call should return all black (empty)
    let img = ssw.capture_screen().unwrap();
    assert!(ssw.is_image_empty(&img), "First call should return empty black image");
    
    // Second call should return all white (empty)
    let img = ssw.capture_screen().unwrap();
    assert!(ssw.is_image_empty(&img), "Second call should return empty white image");
    
    // Third call should return image with red pixel (not empty)
    let img = ssw.capture_screen().unwrap();
    assert!(!ssw.is_image_empty(&img), "Third call should return non-empty image");
    
    // Test write_screenshot
    assert!(ssw.write_screenshot().is_ok(), "write_screenshot should succeed");
    
    // Test write_screenshot multiple times
    for _ in 0..5 {
        assert!(ssw.write_screenshot().is_ok(), "write_screenshot should succeed multiple times");
    }
    
    // Test write_screenshot with an error in capture_screen
    let mut ssw_err = TestScreenshotWriter::new();
    let _ = ssw_err.call_count.store(100, std::sync::atomic::Ordering::SeqCst);
    assert!(ssw_err.write_screenshot().is_err(), "write_screenshot should fail with error in capture_screen");
}
