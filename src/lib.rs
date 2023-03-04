
#![feature(absolute_path)]
use chrono::Local;
use log::{info};
use image::ImageError;
use screenshots::{DisplayInfo, Image, Screen};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{self, PathBuf};

fn capture_screen() -> Result<Image, ImageError> {
    let di = DisplayInfo::from_point(0, 0).unwrap();
    let screen = Screen::new(&di);
    let image = screen.capture().unwrap();
    return Ok(image);
}

pub struct ScreenshotWriter {
    write_folder: PathBuf,
    last_screenshot: Image,
}

impl ScreenshotWriter {
    pub fn new(folder: PathBuf) -> Self {
        return ScreenshotWriter {
            write_folder: path::absolute(PathBuf::from(&folder)).unwrap(),
            last_screenshot: Image::new(0, 0, Vec::new()),
        }
    }
    
    pub fn write_screenshot(&mut self) {
        fs::create_dir_all(self.full_path_date_folder()).expect("Failed to create directory.");
        let full_path = self.full_path_date_folder().join(Self::current_time_image_filename());
        let image = capture_screen().unwrap();
        let mut file = File::create(&full_path).unwrap();
        file.write_all(image.buffer()).unwrap();
        self.last_screenshot = image;
        info!("Wrote screenshot {}", &full_path.display());
    }
    
    pub fn write_folder(&self) -> &PathBuf {
        &self.write_folder
    }

    pub fn full_path_date_folder(&self) -> PathBuf {
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
