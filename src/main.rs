use win_screenshot::capture::*;
use std::time::Duration;
use std::thread;
use ctrlc;

fn main() {
    // set up Ctrl-C handling
    ctrlc::set_handler(|| {
        // this code will be executed when the user hits Ctrl-C
        println!("Ctrl-C pressed!");

        // exit the program
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    let mut i = 0;
    loop {
        i = i + 1;
        // create a new thread to run the job
        let handle = thread::spawn(move || {
            let mut filename = String::from("screenshot");
            filename.push_str(&i.to_string());
            filename.push_str(".jpg");
            println!("**** Writing filename: {}", filename);
            let image = capture_display().unwrap();
            image.save(filename).unwrap();
            println!("Running job...{}", i);
        });

        // sleep for 10 seconds
        thread::sleep(Duration::from_secs(10));

        // wait for the job to finish
        handle.join().unwrap();
    }
}
