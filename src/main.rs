#![allow(unused)]
use std::{env, process::Command, thread, time::Duration};

fn main() {
    // Get commands from the CLI
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: rwatch <seconds> <command>");
        println!("Example: rwatch 2 \"netstat -ant\" ");
        return;
    }

    let interval: u64 = args[1]
        .parse()
        .expect("Please provide a number for seconds");

    let cmd_to_run = &args[2];

    println!("Watching: {} every {}s", cmd_to_run, interval);

    loop {
        print!("\x1B[2J\x1B[1;1H");
        println!("R-WATCH | Command: {} | Every {}s", cmd_to_run, interval);
        println!("--------------------------------------------------");

        // Command executions
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd").args(["/C", cmd_to_run]).output()
        } else {
            Command::new("sh").args(["-c", cmd_to_run]).output()
        };

        // Printing the results
        match output {
            Ok(out) => {
                println!("{}", String::from_utf8_lossy(&out.stdout));
                if !out.stderr.is_empty() {
                    println!("Error : {}", String::from_utf8_lossy(&out.stderr));
                }
            }
            Err(e) => println!("Failed to execute: {}", e),
        }

        // Wait before the next loop
        thread::sleep(Duration::from_secs(interval));
    }
}
