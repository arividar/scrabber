#![feature(absolute_path)]
use clap::Parser;
use ctrlc;
use log::{debug};
use scrabber::ScreenshotWriter;
use std::env;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

#[cfg(test)]
use {
    std::fs,
    std::path,
    chrono::Local,
    serial_test::serial,
    std::fs::read_dir,
    tempdir::TempDir
};

const RUST_LOG: &str = "RUST_LOG";
const DEFAULT_INTERVAL: u16 = 10;
const DEFAULT_COUNT: u32 = 1;

#[derive(Parser, Debug)]
#[command(author, version = None)]
#[command(about = "Periodically captures a screenshot and saves to a file")]
#[command(
    long_about = "Scrabber a command line utility that captures screenshot periodically
and writes them to a supplied directory. By default the file is named by the current
date and time like so 2027-06-20_10.06.37.png."
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
    write_screenshots(
        cli.path.unwrap_or(String::from(".")),
        cli.interval.unwrap_or(DEFAULT_INTERVAL),
        cli.count.unwrap_or(DEFAULT_COUNT),
        cli.forever,
    );
    debug!("Stopping screen capturing!");
}

fn enable_ctrl_break() {
    ctrlc::set_handler(|| {
        debug!("Capturing stopped by Ctrl-C");
        std::process::exit(0);
    })
    .expect("Ctrl-C handler failure.");
}

fn write_screenshots(path_str: String, interval: u16, count: u32, forever: bool) {
    let mut ssw = ScreenshotWriter::new(PathBuf::from(&path_str));
    let mut times_left = count;
    loop {
        //let _handle = thread::spawn(move || save_screenshot(&full_path));
        ssw.write_screenshot();
        if !forever {
            times_left -= 1;
            if times_left < 1 {
                break;
            }
        }
        thread::sleep(Duration::from_secs(interval as u64));
    }
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
    fn sswriter_constructor_should_set_full_path_write_folder() {
        let tmp_path = PathBuf::from("./");
        let ssr = ScreenshotWriter::new(tmp_path.clone());
        let expected_full_path = path::absolute(PathBuf::from(&tmp_path)).unwrap();
        assert_ne!(tmp_path, expected_full_path);
        assert_eq!(ssr.write_folder(), &expected_full_path);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    fn file_count(folder: &PathBuf) -> u32 {
        read_dir(folder).unwrap().count() as u32
    }

    #[test]
    #[serial]
    fn setloglevel_creates_rustlog_env_variable_if_it_doesnt_exist() {
        env::remove_var(RUST_LOG);
        assert!(env::var(RUST_LOG).is_err());
        set_log_level("");
        assert!(env::var(RUST_LOG).is_ok());
    }

    #[test]
    #[serial]
    fn setloglevel_does_not_change_rustlog_env_var_if_it_exists() {
        const EXPECTED: &str = "bingo";
        env::remove_var(RUST_LOG);
        env::set_var(RUST_LOG, EXPECTED);
        assert_eq!(EXPECTED, env::var(RUST_LOG).unwrap());
        set_log_level("bongo");
        assert_eq!(EXPECTED, env::var(RUST_LOG).unwrap());
    }

    #[test]
    #[serial]
    fn setloglevel_sets_rust_log_env_variable_to_level() {
        const EXPECTED: &str = "warning";
        env::remove_var(RUST_LOG);
        set_log_level(EXPECTED);
        assert_eq!(EXPECTED, env::var(RUST_LOG).unwrap());
    }
    #[test]
    fn should_return_full_path_date_folder() {
        let test_date_str = Local::now().format("%Y-%m-%d").to_string();
        let test_path: PathBuf = env::current_dir().unwrap();
        let expected: PathBuf = path::absolute(&test_path).unwrap().join(test_date_str);
        let ssw = ScreenshotWriter::new(test_path);
        assert_eq!(expected, ssw.full_path_date_folder())
    }

    #[test]
    fn param_count_two_should_create_two_screenshot_files_in_a_subdirectory() {
        let path = path::absolute(TempDir::new("scrabber").unwrap()).unwrap();
        let path_str = path.to_str().unwrap();
        let path_day_folder = path.join(ScreenshotWriter::today_directory_name());
        const EXPECTED: u32 = 2;
        fs::create_dir_all(&path_str).unwrap();
        write_screenshots(String::from(path_str), 0, EXPECTED, false);
        assert!(&path_day_folder.is_dir());
        assert_eq!(EXPECTED, file_count(&path_day_folder));
    }

    #[test]
    fn write_screenshot_should_create_a_file() {
        set_log_level("info");
        let tmp_dir = path::absolute(TempDir::new("scrabber").unwrap().path()).unwrap();
        fs::create_dir_all(&tmp_dir).unwrap();
        let mut ssw = ScreenshotWriter::new(PathBuf::from(&tmp_dir));
        let filename = &PathBuf::from(
            ssw.full_path_date_folder()
                .join(ScreenshotWriter::current_time_image_filename()),
        );
        assert!(!&filename.exists());
        ssw.write_screenshot();
        assert!(&filename.exists());
    }
}
