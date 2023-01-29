#![feature(absolute_path)]
#[allow(dead_code)]
use chrono::Local;
use clap::Parser;
use ctrlc;
use log::info;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

#[cfg(target_os = "windows")]
use win_screenshot::capture::*;

#[cfg(target_os = "macos")]
use image::{ImageBuffer, ImageError, Rgba};

#[cfg(target_os = "macos")]
use rand;

const RUST_LOG: &str = "RUST_LOG";
const DEFAULT_INTERVAL: u16 = 10;

#[derive(Parser, Debug)]
#[command(author, version = None)]
#[command(about = "Periodically captures a screenshot and saves to a file")]
#[command(
    long_about = "Captures a screenshot of the current screen and stores it as jpg-file in the 
supplied directory. By default the file is named by the current date and time 
like so 2027-06-20_10.06.37.jpg."
)]
pub struct Cli {
    /// Optional path of a folder where to put the screenshot files
    #[arg(short, long, value_name = "PATH")]
    path: Option<String>,
    /// Optional interval in seconds between creating a new screenshot
    #[arg(short, long, value_name = "INTERVAL")]
    interval: Option<u16>,
}

fn main() {
    set_log_level("debug");
    env_logger::init();
    info!("Starting screen capturing!");

    let mut interval: u16 = DEFAULT_INTERVAL;
    let mut path: PathBuf = PathBuf::from("");

    parse_cli_params(&mut path, &mut interval);
    enable_ctrl_break();
    write_files_until_break(&path, &interval);

    info!("Stopping screen capturing!");
}

fn enable_ctrl_break() {
    ctrlc::set_handler(|| {
        info!("Capturing stopped by Ctrl-C");
        std::process::exit(0);
    })
    .expect("Ctrl-C handler failure.");
}

fn parse_cli_params(path: &mut PathBuf, interval: &mut u16) {
    let cli: Cli = Cli::parse();
    *interval = cli.interval.unwrap_or(DEFAULT_INTERVAL);
    *path = std::path::absolute(PathBuf::from(cli.path.unwrap_or(String::from(".")))).unwrap();
}

fn write_files_until_break(path: &PathBuf, interval: &u16) {
    if !std::path::Path::new(path).exists() {
        fs::create_dir_all(path).expect("Failed to create directory.");
    }
    loop {
        let filename: String = Local::now().format("%Y-%m-%dT%H.%M.%S").to_string() + ".jpg";
        let fullpath = path.join(&filename);
        let image = capture_screen().unwrap();
        image.save(&fullpath).unwrap();
        info!("Saved screenshot {}", &fullpath.display());
        thread::sleep(Duration::from_secs(*interval as u64));
    }
}

#[cfg(target_os = "windows")]
fn capture_screen() -> Result<Image, WSError> {
    capture_display()
}

#[cfg(target_os = "macos")]
fn capture_screen() -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, ImageError> {
    let mut img = ImageBuffer::new(800, 600);
    for (_x, _y, pixel) in img.enumerate_pixels_mut() {
        *pixel = image::Rgba([
            rand::random::<u8>(),
            rand::random::<u8>(),
            rand::random::<u8>(),
            127,
        ]);
    }
    return Ok(img);
}

fn set_log_level(loglevel: &str) {
    if env::var(RUST_LOG).is_err() {
        env::set_var(RUST_LOG, loglevel);
    }
}

#[cfg(test)]
mod tests {
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
