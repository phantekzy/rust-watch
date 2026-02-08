#![allow(unused)]
use std::env;

fn main() {
    // Get commands from the CLI
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: rwatch <seconds> <command>");
        println!("Example: rwatch 2 \"netstat -ant\" ");
        return;
    }
}
