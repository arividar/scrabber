use chrono::{DateTime, Local};
use clap::Parser;
use log::{ info, warn, debug };
use ctrlc;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use win_screenshot::capture::*;

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
    #[arg(short, long, value_name = "FOLDER")]
    path: Option<PathBuf>,
    /// Optional filename to save the screenshot to
    #[arg(short, long, value_name = "FILENAME")]
    filename: Option<String>,
    /// The Interval in seconds between creating a new screenshot
    #[arg(short, long, value_name = "INTERVAL")]
    interval: Option<u8>,
}

fn main() {
    env_logger::init();
    info!("Starting screen capturing!");
    extract_cli_params();
    enable_ctrl_c_break();
    write_files_until_break();
    info!("Stopping screen capturing!");
}

fn extract_cli_params() {
    let _cli = Cli::parse();
}

fn enable_ctrl_c_break() {
    ctrlc::set_handler(|| {
        warn!("**** þú smelltir á Ctrl-C");
        std::process::exit(0);
    })
    .expect("Ctrl-C handler failure.");
}

fn write_files_until_break() {
    loop {
        let handle = thread::spawn(|| {
            let now: DateTime<Local> = Local::now();
            let mut filename = now.format("%Y-%m-%d_%H%M%S").to_string();
            filename.push_str(".jpg");
            let image = capture_display().unwrap();
            image.save(&filename).unwrap();
            debug!("Saved image {:?}.", filename);
        });
        thread::sleep(Duration::from_secs(10));
        handle.join().unwrap();
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
}
