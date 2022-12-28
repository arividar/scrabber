use win_screenshot::capture::*;
use std::time::Duration;
use std::path::PathBuf;
use std::thread;
use ctrlc;
use chrono::{Local, DateTime};
use clap::{Parser};

#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional path of a folder where to put the screenshot files
    #[arg(short, long, value_name = "FOLDER")]
    path: Option<PathBuf>,

    /// The filename to save the screenshot to
    #[arg(short, long, value_name = "FILENAME")]
    filename: String,

    /// The Interval in seconds between creating a new screenshot
    #[arg(short, long, action = clap::ArgAction::Count)]
    interval: u8,

}

fn main() {
    let cli = Cli::parse();
    println!("**** Cli={:?}", cli);
    ctrlc::set_handler(|| {
        // this code will be executed when the user hits Ctrl-C
        println!("Ctrl-C pressed!");

        // exit the program
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    loop {
        // create a new thread to run the job
        let handle = thread::spawn(|| {
            let now: DateTime<Local> = Local::now();
            let mut filename = now.format("%Y-%m-%d_%H%M%S").to_string();
            println!("The current date and time is: {}", &filename);
            filename.push_str(".jpg");
            let image = capture_display().unwrap();
            image.save(&filename).unwrap();
            println!("**** Wrote filename: {}", &filename);
        });

        // sleep for 10 seconds
        thread::sleep(Duration::from_secs(10));

        // wait for the job to finish
        handle.join().unwrap();
    }
}
