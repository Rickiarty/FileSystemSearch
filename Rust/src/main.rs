use std::env;
use std::process;

mod search;
mod format_number;

use crate::search::{search_under_path, VERSION};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 2 {
        eprintln!("\nERROR: Invalid argument(s) !\n\nType either '--help' or '-h' for more info.\n");
        process::exit(-6);
    } else if args.len() < 2 {
        eprintln!("\nERROR: Invalid argument(s) !\n\nType either '--help' or '-h' for more info.\n");
        process::exit(-6);
    } else {
        match args[1].as_str() {
            "--version" => {
                println!("\n\tversion: {}\n\t\tby Rei-Chi Lin\n", VERSION);
                process::exit(2);
            }
            "--help" | "-h" => {
                println!("\nusage:\n\n\t Give this program a regular expression to search under current path. \n");
                process::exit(1);
            }
            _ => {
                let regular_expression = &args[1];
                let current_dir = env::current_dir().unwrap();
                match search_under_path(regular_expression, &current_dir) {
                    Ok(code) => process::exit(code),
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        process::exit(-1);
                    }
                }
            }
        }
    }
}