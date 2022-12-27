use win_screenshot::capture::*;
use std::time::Duration;
use std::thread;
use ctrlc;
use chrono::{Local, DateTime};

fn main() {
    // set up Ctrl-C handling
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
            let mut filename = now.format("%Y-%m-%dT%H%M%S").to_string();
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
