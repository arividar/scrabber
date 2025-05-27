use std::env;
// std::fs is not directly used here, create_dir_all is called with std::fs::create_dir_all
use std::path::PathBuf;
use std::sync::{Arc, atomic::AtomicUsize};

use chrono::Local;
use clap::{Arg, ArgMatches, Command};
use image::{Rgba, RgbaImage}; // Removed ImageBuffer as RgbaImage is used directly
use screenshots::{Screen, Image};

use scrabber::{ScreenshotWriter, ScreenshotError, is_image_empty};

fn setup_panic_hook() {
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("Panic occurred: {:?}", panic_info);
    }));
}

fn set_log_level(level_str: &str) {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", level_str);
    }
    env_logger::init();
}

fn run_program_logic(matches: ArgMatches) -> Result<(), ScreenshotError> {
    let path_str = matches.get_one::<String>("path").expect("Path is required");
    let count = *matches.get_one::<u32>("count").expect("Count is required");
    let interval = *matches.get_one::<u64>("interval").expect("Interval is required");

    let mut ssw: DefaultScreenshotWriter = DefaultScreenshotWriter::new(PathBuf::from(&path_str));

    for i in 0..count {
        ssw.write_screenshot()?;
        if i < count - 1 {
            std::thread::sleep(std::time::Duration::from_secs(interval));
        }
    }
    Ok(())
}

fn main() {
    setup_panic_hook();
    set_log_level("info"); 

    let matches = Command::new("Scrabber")
        .version("0.1.0")
        .author("Your Name <you@example.com>")
        .about("Takes screenshots at specified intervals")
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .value_name("PATH")
                .help("Sets the path to save screenshots")
                .required(true)
                .num_args(1),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .value_name("COUNT")
                .help("Number of screenshots to take")
                .value_parser(clap::value_parser!(u32))
                .required(true)
                .num_args(1),
        )
        .arg(
            Arg::new("interval")
                .short('i')
                .long("interval")
                .value_name("INTERVAL")
                .help("Interval in seconds between screenshots")
                .value_parser(clap::value_parser!(u64))
                .required(true)
                .num_args(1),
        )
        .get_matches();

    if let Err(e) = run_program_logic(matches) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

struct DefaultScreenshotWriter {
    path: PathBuf,
}

impl DefaultScreenshotWriter {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl ScreenshotWriter for DefaultScreenshotWriter {
    fn new(path: PathBuf) -> Self where Self: Sized {
        DefaultScreenshotWriter { path }
    }

    fn capture_screen(&self) -> Result<Image, ScreenshotError> { 
        let screens = Screen::all().map_err(|e| ScreenshotError::Other(format!("Failed to get screen list: {}", e)))?;
        let screen = screens.first().ok_or_else(|| ScreenshotError::Other("No screens found".to_string()))?;
        screen.capture().map_err(|e| ScreenshotError::Other(format!("Failed to capture screen: {}", e)))
    }

    fn is_image_empty(&self, image: &Image) -> bool { 
        is_image_empty(image)
    }

    fn date_folder_path(&self) -> PathBuf {
        let current_date = chrono::Local::now().format("%Y-%m-%d").to_string();
        self.path.join(current_date)
    }

    fn write_screenshot(&mut self) -> Result<(), ScreenshotError> {
        let screen_image = self.capture_screen()?;

        if self.is_image_empty(&screen_image) {
            println!("Image is empty, not saving.");
            return Ok(());
        }

        let date_folder = self.date_folder_path();
        std::fs::create_dir_all(&date_folder).map_err(|e| {
            eprintln!("Failed to create directory: {} for path {:?}", e, date_folder);
            ScreenshotError::IoError(e)
        })?;

        let timestamp = chrono::Local::now().format("%H-%M-%S%.3f").to_string();
        let filename = format!("screenshot-{}.png", timestamp);
        let file_path = date_folder.join(filename);

        let rgba_image = RgbaImage::from_raw(screen_image.width(), screen_image.height(), screen_image.buffer().to_vec())
            .ok_or_else(|| ScreenshotError::Other("Failed to convert screenshot to RgbaImage".to_string()))?;

        rgba_image.save(&file_path).map_err(|e| {
            eprintln!("Failed to save image: {} to path {:?}", e, file_path);
            ScreenshotError::ImageError(e)
        })?;

        println!("Screenshot saved to {:?}", file_path);
        Ok(())
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*; 
    use image::Rgba; 

    fn rgba_to_screenshots_image(rgba_img: &RgbaImage) -> Image {
        Image::new(rgba_img.width(), rgba_img.height(), rgba_img.as_raw().to_vec())
    }

    #[test]
    fn is_image_empty_should_detect_all_black_image() {
        let width = 2;
        let height = 2;
        let mut image_data = RgbaImage::new(width, height);
        for y in 0..height {
            for x in 0..width {
                image_data.put_pixel(x, y, Rgba([0, 0, 0, 255]));
            }
        }
        let ssw = DefaultScreenshotWriter::new(PathBuf::from("/tmp"));
        assert!(ssw.is_image_empty(&rgba_to_screenshots_image(&image_data)));
    }

    #[test]
    fn is_image_empty_should_detect_all_white_image() {
        let width = 2;
        let height = 2;
        let mut image_data = RgbaImage::new(width, height);
        for y in 0..height {
            for x in 0..width {
                image_data.put_pixel(x, y, Rgba([255, 255, 255, 255]));
            }
        }
        let ssw = DefaultScreenshotWriter::new(PathBuf::from("/tmp"));
        assert!(ssw.is_image_empty(&rgba_to_screenshots_image(&image_data)));
    }

    #[test]
    fn is_image_empty_should_detect_mixed_content_image() {
        let width = 2;
        let height = 2;
        let mut image_data = RgbaImage::new(width, height);
        image_data.put_pixel(0, 0, Rgba([0, 0, 0, 255]));
        image_data.put_pixel(1, 0, Rgba([255, 255, 255, 255]));
        image_data.put_pixel(0, 1, Rgba([0, 0, 0, 255]));
        image_data.put_pixel(1, 1, Rgba([0, 0, 0, 255])); 
        let ssw = DefaultScreenshotWriter::new(PathBuf::from("/tmp"));
        assert!(!ssw.is_image_empty(&rgba_to_screenshots_image(&image_data)));
    }
/*
    #[test]
    fn sswriter_constructor_should_set_full_path_write_folder() {
        let tmp_path = PathBuf::from("./");
        let ssr = ScreenshotWriter::new(tmp_path.clone());
        let expected_full_path = path::absolute(PathBuf::from(&tmp_path)).unwrap();
        assert_ne!(tmp_path, expected_full_path);
        assert_eq!(ssr.write_folder(), &expected_full_path);
    }
*/
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use chrono::Local;
    use scrabber::ScreenshotWriter; 
    use scrabber::ScreenshotError;
    use tempfile::tempdir;

    struct TestScreenshotWriter {
        path: PathBuf,
        test_image_path: PathBuf, 
    }

    impl TestScreenshotWriter {
        fn new(path: PathBuf, test_image_path: PathBuf) -> Self {
            Self { path, test_image_path }
        }
    }

    impl ScreenshotWriter for TestScreenshotWriter {
        fn new(_path: PathBuf) -> Self where Self: Sized { 
            panic!("TestScreenshotWriter needs a test_image_path, use specific constructor");
        }

        fn capture_screen(&self) -> Result<Image, ScreenshotError> { 
            let rgba_image = image::open(&self.test_image_path)
                .map_err(|e| ScreenshotError::ImageError(e))?
                .to_rgba8();
            Ok(Image::new(rgba_image.width(), rgba_image.height(), rgba_image.into_raw()))
        }

        fn is_image_empty(&self, image: &Image) -> bool { 
            is_image_empty(image)
        }

        fn date_folder_path(&self) -> PathBuf {
            self.path.join(Local::now().format("%Y-%m-%d").to_string())
        }
        
        fn write_screenshot(&mut self) -> Result<(), ScreenshotError> {
            let screen_image = self.capture_screen()?;
            if !self.is_image_empty(&screen_image) {
                let date_folder = self.date_folder_path();
                fs::create_dir_all(&date_folder)
                    .map_err(|e| ScreenshotError::IoError(e))?;
                let timestamp = chrono::Local::now().format("%H-%M-%S%.3f").to_string();
                let file_path = date_folder.join(format!("screenshot_{}.png", timestamp));

                let rgba_image = RgbaImage::from_raw(screen_image.width(), screen_image.height(), screen_image.buffer().to_vec())
                    .ok_or_else(|| ScreenshotError::Other("Failed to convert screenshot to RgbaImage for TestScreenshotWriter".to_string()))?;
                
                rgba_image.save(&file_path)
                    .map_err(|e| ScreenshotError::ImageError(e))?;
            }
            Ok(())
        }
    }

    #[test]
    fn param_count_two_should_create_two_screenshot_files_in_a_subdirectory() {
        set_log_level("info");
        let test_dir_base = tempdir().unwrap().path().to_path_buf(); 
        fs::create_dir_all(&test_dir_base).unwrap();

        let mut image_data = RgbaImage::new(100, 100);
        for x in 0..10 {
            for y in 0..10 {
                image_data.put_pixel(x, y, Rgba([255, 0, 0, 255]));
            }
        }
        let test_image_file = test_dir_base.join("temp_test_image.png");
        image_data.save(&test_image_file).unwrap();

        let mut test_ssw = TestScreenshotWriter::new(test_dir_base.clone(), test_image_file.clone());

        test_ssw.write_screenshot().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10)); 
        test_ssw.write_screenshot().unwrap();

        let day_folder = test_ssw.date_folder_path();
        assert!(day_folder.is_dir());
        
        let mut count = 0;
        for entry in fs::read_dir(day_folder).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() {
                count += 1;
            }
        }
        assert_eq!(2, count, "Expected two files in the date folder");
        // temp_dir will clean up automatically on drop
    }
}

struct MockScreenshotWriter {
    path: PathBuf,
    call_count: std::sync::atomic::AtomicUsize,
    black_image: Arc<Image>, 
    white_image: Arc<Image>,
    mixed_image: Arc<Image>,
    image_to_return: Option<Arc<Image>>, 
    force_empty: bool, 
}

impl MockScreenshotWriter {
    fn new(path: PathBuf) -> Self {
        let width = 2;
        let height = 2;
        
        let mut black_rgba = RgbaImage::new(width, height);
        for p in black_rgba.pixels_mut() { *p = Rgba([0,0,0,255]); }
        let black_image = Arc::new(Image::new(width, height, black_rgba.into_raw()));
        
        let mut white_rgba = RgbaImage::new(width, height);
        for p in white_rgba.pixels_mut() { *p = Rgba([255,255,255,255]); }
        let white_image = Arc::new(Image::new(width, height, white_rgba.into_raw()));
        
        let mut mixed_rgba = RgbaImage::new(width, height);
        mixed_rgba.put_pixel(0, 0, Rgba([0, 0, 0, 255]));
        mixed_rgba.put_pixel(1, 0, Rgba([255, 255, 255, 255]));
        mixed_rgba.put_pixel(0, 1, Rgba([0, 0, 0, 255]));
        mixed_rgba.put_pixel(1, 1, Rgba([255, 0, 0, 255])); 
        let mixed_image = Arc::new(Image::new(width, height, mixed_rgba.into_raw()));
        
        Self {
            path,
            call_count: AtomicUsize::new(0),
            black_image,
            white_image,
            mixed_image,
            image_to_return: None,
            force_empty: false,
        }
    }
    
    fn get_image_clone(&self, arc_image: &Arc<Image>) -> Image {
        Image::new(arc_image.width(), arc_image.height(), arc_image.buffer().to_vec())
    }

    #[allow(dead_code)] 
    fn set_image_to_return(&mut self, image: Image) {
        self.image_to_return = Some(Arc::new(image));
    }

    #[allow(dead_code)]
    fn set_empty_image(&mut self, empty: bool) {
        self.force_empty = empty;
    }
}

impl ScreenshotWriter for MockScreenshotWriter {
    fn new(path: PathBuf) -> Self where Self: Sized {
        MockScreenshotWriter::new(path)
    }

    fn capture_screen(&self) -> Result<Image, ScreenshotError> { 
        if let Some(ref img_arc) = self.image_to_return {
            return Ok(self.get_image_clone(img_arc));
        }
        let count = self.call_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        match count % 3 { 
            0 => Ok(self.get_image_clone(&self.black_image)),
            1 => Ok(self.get_image_clone(&self.white_image)),
            _ => Ok(self.get_image_clone(&self.mixed_image)),
        }
    }

    fn is_image_empty(&self, image: &Image) -> bool { 
        if self.force_empty {
            return true;
        }
        is_image_empty(image)
    }
    
    fn date_folder_path(&self) -> PathBuf {
        self.path.join(Local::now().format("%Y-%m-%d").to_string())
    }

    fn write_screenshot(&mut self) -> Result<(), ScreenshotError> {
        let screen_image = self.capture_screen()?;
        if !self.is_image_empty(&screen_image) {
            let date_folder = self.date_folder_path();
            std::fs::create_dir_all(&date_folder).map_err(|e| {
                eprintln!("Failed to create directory: {}", e);
                ScreenshotError::IoError(e)
            })?;
            
            let timestamp = Local::now().format("%H-%M-%S%.3f").to_string();
            let file_path = date_folder.join(format!("mock_screenshot_{}.png", timestamp));

            let rgba_image = RgbaImage::from_raw(screen_image.width(), screen_image.height(), screen_image.buffer().to_vec())
                .ok_or_else(|| ScreenshotError::Other("Failed to convert screenshot to RgbaImage for MockScreenshotWriter".to_string()))?;

            rgba_image.save(&file_path).map_err(|e| {
                eprintln!("Failed to save image: {}", e);
                ScreenshotError::ImageError(e)
            })?;
        }
        Ok(())
    }
}
