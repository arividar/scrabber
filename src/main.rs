use chrono::{DateTime, Local};
use clap::Parser;
use ctrlc;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use win_screenshot::capture::*;

#[cfg(test)]
mod winscreenshottests;

#[derive(Parser, Debug)]
#[command(name = "winscreenshot")]
#[command(author = "Ari Johannesson <arividar@gmail.com>")]
#[command(version = "0.1")]
#[command(about = "Periodically captures a screenshot and saves to a file")]
#[command(
    long_about = "Captures a screenshot of the current screen and stores it as jpg-file in the 
supplied directory. By default the file is named by the current date and time 
like so 2027-06-20_10.06.37.jpg."
)]

struct Cli {
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
    let _cli = Cli::parse();
    println!("CLI er svona: {:?}", _cli);

    ctrlc::set_handler(|| {
        std::process::exit(0);
    })
    .expect("Ctrl-C handler failure.");

    loop {
        let handle = thread::spawn(|| {
            let now: DateTime<Local> = Local::now();
            let mut filename = now.format("%Y-%m-%d_%H%M%S").to_string();
            println!("The current date and time is: {}", &filename);
            filename.push_str(".jpg");
            let image = capture_display().unwrap();
            image.save(&filename).unwrap();
            println!("**** Wrote filename: {}", &filename);
        });
        thread::sleep(Duration::from_secs(10));
        handle.join().unwrap();
    }
}
