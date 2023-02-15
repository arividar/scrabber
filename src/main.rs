#![feature(absolute_path)] #[allow(dead_code)]
use chrono::Local;
use clap::Parser;
use ctrlc;
use image::ImageError;
use log::{debug,info};
use screenshots::{DisplayInfo, Image, Screen};
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

const FOREVER: u32 = std::u32::MAX;
const RUST_LOG: &str = "RUST_LOG";
const DEFAULT_INTERVAL: u16 = 10;

#[derive(Parser, Debug)]
#[command(author, version = None)]
#[command(about = "Periodically captures a screenshot and saves to a file")]
#[command(
    long_about = "Captures a screenshot of the current screen and stores it as png-file in the 
supplied directory. By default the file is named by the current date and time 
like so 2027-06-20_10.06.37.png."
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
}

fn main() {
    set_log_level("info");
    env_logger::init();
    debug!("Starting screen capturing!");

    let mut interval: u16 = DEFAULT_INTERVAL;
    let mut count: u32 = FOREVER;
    let mut path: PathBuf = PathBuf::from("");

    parse_cli_params(&mut path, &mut interval, &mut count);
    enable_ctrl_break();
    write_files_until_break(&path, &interval, &count);

    debug!("Stopping screen capturing!");
}

fn enable_ctrl_break() {
    ctrlc::set_handler(|| {
        info!("Capturing stopped by Ctrl-C");
        std::process::exit(0);
    })
    .expect("Ctrl-C handler failure.");
}

fn parse_cli_params(path: &mut PathBuf, interval: &mut u16, count: &mut u32) {
    let cli: Cli = Cli::parse();
    *path = std::path::absolute(PathBuf::from(cli.path.unwrap_or(String::from(".")))).unwrap();
    *interval = cli.interval.unwrap_or(DEFAULT_INTERVAL);
    *count = cli.count.unwrap_or(FOREVER);
}

fn write_files_until_break(path: &PathBuf, interval: &u16, count: &u32) {
    let mut times_left = *count;
    loop {
        let full_path = create_timed_file_full_path(&path);
        //let _handle = thread::spawn(move || save_screenshot(&full_path));
        save_screenshot(&full_path);
        if *count != FOREVER {
            times_left -= 1;
            if times_left < 1 {
                break;
            }
        }
        thread::sleep(Duration::from_secs(*interval as u64));
    }
}

fn create_timed_file_full_path(path: &PathBuf) -> PathBuf {
    let daypath: PathBuf = path.join(Local::now().format("%Y-%m-%d").to_string());
    fs::create_dir_all(&daypath).expect("Failed to create directory.");
    let filename: String = Local::now().format("%Y-%m-%dT%H.%M.%S").to_string() + ".png";
    daypath.join(&filename)
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
    debug!("capturer {:?}", screen);
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
        assert!(env::var(RUST_LOG).is_err());
        set_log_level(EXPECTED);
        assert!(env::var(RUST_LOG).unwrap() == EXPECTED);
    }
}


#[cfg(test)]
mod integration_tests {
    use super::*;
    #[test]
    fn tbd() {
        assert!(true);
    }
}
