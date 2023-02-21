#![feature(absolute_path)]
#[allow(dead_code)]
use chrono::Local;
use clap::Parser;
use ctrlc;
use image::ImageError;
use log::{debug, info};
use screenshots::{DisplayInfo, Image, Screen};
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{self, PathBuf};
use std::thread;
use std::time::Duration;
#[cfg(test)]
use tempdir::TempDir;

const RUST_LOG: &str = "RUST_LOG";
const DEFAULT_INTERVAL: u16 = 10;
const DEFAULT_COUNT: u32 = 1;

#[derive(Parser, Debug)]
#[command(author, version = None)]
#[command(about = "Periodically captures a screenshot and saves to a file")]
#[command(
    long_about = "Scrabber captures a screenshot and saves them to png-files in the supplied 
directory. By default the file is named by the current date and time like 
so 2027-06-20_10.06.37.png."
)]

pub struct Cli {
    /// Optional path of a folder where to put the screenshot files
    #[arg(short, long, value_name = "PATH")]
    path: Option<String>,
    /// Optional interval in seconds between creating a new screenshot
    #[arg(short, long, value_name = "NUMBER")]
    interval: Option<u16>,
    /// Optional count of how many screenshots to take
    #[arg(short, long, value_name = "NUMBER")]
    count: Option<u32>,
    /// If set the screen capture will run continuously until stopped
    /// with Ctrl-C. Overrides the count parameter.
    #[arg(short, long, action)]
    forever: bool,
}

fn main() {
    set_log_level("info");
    env_logger::init();
    debug!("Starting screen capturing!");
    enable_ctrl_break();

    let cli: Cli = Cli::parse();
    let path = PathBuf::from(cli.path.unwrap_or(String::from(".")));
    let interval = cli.interval.unwrap_or(DEFAULT_INTERVAL);
    let count = cli.count.unwrap_or(DEFAULT_COUNT);
    let forever = cli.forever;

    write_screenshots(path, interval, count, forever);

    debug!("Stopping screen capturing!");
}

fn enable_ctrl_break() {
    ctrlc::set_handler(|| {
        debug!("Capturing stopped by Ctrl-C");
        std::process::exit(0);
    })
    .expect("Ctrl-C handler failure.");
}

fn write_screenshots(p: PathBuf, interval: u16, count: u32, forever: bool) {
    let mut times_left = count;
    loop {
        let date_folder_path = current_date_folder(&p);
        fs::create_dir_all(&date_folder_path).expect("Failed to create directory.");
        let full_path = date_folder_path
                        .join(Local::now().format("%Y-%m-%dT%H.%M.%S").to_string() + ".png");
        //let _handle = thread::spawn(move || save_screenshot(&full_path));
        save_screenshot(&full_path);
        if !forever {
            times_left -= 1;
            if times_left < 1 {
                break;
            }
        }
        thread::sleep(Duration::from_secs(interval as u64));
    }
}

fn current_date_folder(p: &PathBuf) -> PathBuf {
   path::absolute(PathBuf::from(p)).unwrap()
         .join(Local::now().format("%Y-%m-%d").to_string())
}

fn save_screenshot(filename: &PathBuf) {
    let image = capture_screen().unwrap();
    let mut file = File::create(&filename).unwrap();
    file.write_all(image.buffer()).unwrap();
    info!("Saved screenshot {}", &filename.display());
}

fn capture_screen() -> Result<Image, ImageError> {
    let di = DisplayInfo::from_point(0, 0).unwrap();
    let screen = Screen::new(&di);
    let image = screen.capture().unwrap();
    return Ok(image);
}

fn set_log_level(loglevel: &str) {
    if env::var(RUST_LOG).is_err() {
        env::set_var(RUST_LOG, loglevel);
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn main_creates_path_parameter_in_cli() {
        let cli = Cli::parse();
        assert_eq!(cli.path, None);
    }

    #[test]
    fn setloglevel_creates_rustlog_env_variable_if_it_doesnt_exist() {
        env::remove_var(RUST_LOG);
        assert!(env::var(RUST_LOG).is_err());
        set_log_level("");
        assert!(env::var(RUST_LOG).is_ok());
    }

    #[test]
    fn setloglevel_does_not_change_rustlog_env_var_if_it_exists() {
        const EXPECTED: &str = "bingo";
        env::remove_var(RUST_LOG);
        env::set_var(RUST_LOG, EXPECTED);
        assert!(env::var(RUST_LOG).unwrap() == EXPECTED);
        set_log_level("bongo");
        assert!(env::var(RUST_LOG).unwrap() == EXPECTED);
    }

    #[test]
    fn setloglevel_sets_rust_log_env_variable_to_level() {
        const EXPECTED: &str = "warning";
        env::remove_var(RUST_LOG);
        set_log_level(EXPECTED);
        assert!(env::var(RUST_LOG).unwrap() == EXPECTED);
    }
    
    #[test]
    fn test_current_date_folder() {
        //  path::absolute(PathBuf::from(p)).unwrap()
        //        .join(Local::now().format("%Y-%m-%d").to_string())
        const EXPECTED: &str = env::current_dir();
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    #[ignore]
    #[test]
    fn create_timed_file_full_path_should_create_full_path() {
        const EXPECTED: &str = "tbd";
        let _tmp_dir = TempDir::new("example").unwrap();
        assert_eq!(EXPECTED, "FAIL")
    }
}
