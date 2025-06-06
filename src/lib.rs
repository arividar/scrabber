use chrono::Local;
use log::{info};
use xcap::Monitor;
use std::fs::{self, File};
use std::io::{Write, Cursor};
use std::path::{self, PathBuf};

pub struct ScreenshotWriter {
    write_folder: PathBuf,
    last_screenshot: Option<Vec<u8>>,
}

impl ScreenshotWriter {

    pub fn new(folder: PathBuf) -> Self {
        return ScreenshotWriter {
            write_folder: path::absolute(PathBuf::from(&folder)).unwrap(),
            last_screenshot: None,
        }
    }
    
    fn images_are_identical(&self, new_image: &[u8]) -> bool {
        if let Some(ref last) = self.last_screenshot {
            last == new_image
        } else {
            false
        }
    }
    
    fn capture_screen() -> Vec<u8> { 
        let monitors = Monitor::all().expect("Failed to get monitors list");
        
        let target_monitor = monitors
            .iter() 
            .find(|m| m.is_primary().unwrap_or(false)) 
            .or_else(|| monitors.first()) 
            .expect("No monitors found or primary monitor detection failed");

        let image = target_monitor.capture_image().expect("Failed to capture image");
        
        // Use xcap's built-in PNG encoding
        let mut buffer = Cursor::new(Vec::new());
        image.write_to(&mut buffer, xcap::image::ImageFormat::Png)
            .expect("Failed to encode image as PNG");
        buffer.into_inner()
    }

    pub fn write_screenshot(&mut self, skip_duplicates: bool) {
        let image_buffer = Self::capture_screen();
        
        if skip_duplicates && self.images_are_identical(&image_buffer) {
            info!("Skipping duplicate screenshot");
            return;
        }
        
        fs::create_dir_all(self.date_folder_path()).expect("Failed to create directory.");
        let full_path = self.date_folder_path().join(Self::current_time_image_filename());
        let mut file = File::create(&full_path).unwrap();
        file.write_all(&image_buffer).unwrap();
        self.last_screenshot = Some(image_buffer);
        info!("Wrote screenshot {}", &full_path.display());
    }
    
    pub fn write_folder(&self) -> &PathBuf {
        &self.write_folder
    }

    pub fn date_folder_path(&self) -> PathBuf {
        path::absolute(PathBuf::from(&self.write_folder))
            .unwrap()
            .join(Self::today_directory_name())
    }

    pub fn today_directory_name() -> String {
        Local::now().format("%Y-%m-%d").to_string()
    }

    pub fn current_time_image_filename() -> String {
        Local::now().format("%Y-%m-%dT%H.%M.%S").to_string() + ".png"
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use image::ImageFormat;
    use std::io::Cursor;

    #[test]
    fn sswriter_constructor_should_set_full_path_write_folder() {
        let tmp_path = PathBuf::from("./");
        let ssr = ScreenshotWriter::new(tmp_path.clone());
        let expected_full_path = path::absolute(PathBuf::from(&tmp_path)).unwrap();
        assert_ne!(tmp_path, expected_full_path);
        assert_eq!(ssr.write_folder(), &expected_full_path);
    }

    #[test]
    fn captured_screenshot_should_be_valid_png() {
        let image_buffer = ScreenshotWriter::capture_screen();
        
        // Verify the buffer is not empty
        assert!(!image_buffer.is_empty(), "Screenshot buffer should not be empty");
        
        // Try to decode the image using the image crate
        let cursor = Cursor::new(&image_buffer);
        let result = image::load(cursor, ImageFormat::Png);
        
        assert!(result.is_ok(), "Screenshot should be a valid PNG image");
        
        let img = result.unwrap();
        assert!(img.width() > 0, "Image width should be greater than 0");
        assert!(img.height() > 0, "Image height should be greater than 0");
    }

    #[test]
    fn should_skip_duplicate_screenshots_when_enabled() {
        let tmp_path = PathBuf::from("./");
        let mut ssw = ScreenshotWriter::new(tmp_path);
        
        // Simulate having a previous screenshot by setting last_screenshot
        let test_image = vec![1, 2, 3, 4, 5];
        ssw.last_screenshot = Some(test_image.clone());
        
        // Test that identical images are detected
        assert!(ssw.images_are_identical(&test_image));
        
        // Test that different images are not detected as identical
        let different_image = vec![5, 4, 3, 2, 1];
        assert!(!ssw.images_are_identical(&different_image));
        
        // Test with no previous screenshot
        let new_ssw = ScreenshotWriter::new(PathBuf::from("./"));
        assert!(!new_ssw.images_are_identical(&test_image));
    }
}
