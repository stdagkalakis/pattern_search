#[cfg(test)]
mod tests;

use std::fs::File;
use log::error;
use std::path::PathBuf;
use std::io::{BufRead, BufReader};
use anyhow::{Context, Result}; // nicer error handling

// Progress bar using indicatif crate
use std::time::Duration;
use std::thread::sleep;

pub fn search_pattern(pattern: String, path: PathBuf) -> Result<()> {

     // alternative:
    // - use unwrap to exit with panic!
    // - user match to handle the error (similar to try catch in other languages) but using enums
    let file =  File::open(&path)
        .with_context(|| format!("Could not read file {:?}", &path))
        .map_err(|e| {
            error!("Log Error: {}", e);
            e
        })?;

    let mut reader = BufReader::new(file);
    let mut line = String::new();
    
    let mut line_count = 1;
    while let Ok(bytes_read) = reader.read_line(&mut line) {
        if bytes_read == 0 {
            break;
        }

        if line.contains(&pattern) {
            println!("{} : \"{}\"", line_count, line.trim_end());
        }

        line.clear();
        line_count += 1;        
    }

    Ok(())
}

pub fn progress_bar_with_sleep(timer: u64) -> (){

    println!("\nIntresting stuff are happening now, please wait...");
    let timer_value;
    if timer > 100 {
        timer_value = 100;
    } else if timer < 1 {
        timer_value = 10;
    }else {
        timer_value = timer;
    }

    let pb = indicatif::ProgressBar::new(100);
    for _i in 0..100 {
        sleep(Duration::from_millis(timer_value));
        // pb.println( "Allows for information prints for processes..." ));
        pb.inc(1);
    }

    pb.finish_with_message("done");

    adder(1, 2); // this is used to remove warning.
    
    println!("Done!\n");
}

pub fn adder(a: i64, b: i64) -> i64 {
    a + b
}