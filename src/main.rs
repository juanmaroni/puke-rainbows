use std::{env, fs};
use anyhow::{Context, Result};
use puke_rainbows::print_colored;

const DEFAULT_FILE: &str = "lorem-ipsum";

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let num_args = args.len();
    
    if num_args == 0 {
        // Default
        let f = fs::read_to_string(DEFAULT_FILE)
            .with_context(|| format!("could not read file `{}`", DEFAULT_FILE))?;
        print_colored(f);
    } else if num_args == 1 {
        if args[0] == "-h" {
            // Print help message
            let msg = format!("Usage: puke-rainbows {{-h | -t <text> | -f <file>}}");
            print_colored(msg);
        } else {
            // Error
        }
    } else if num_args == 2 {
        if args[0] == "-f" {
            // Read and print file

        } else if args[0] == "-t" {
            // Print text
            
        } else {
            // Error
        }
    } else {
        // Error
    }

    Ok(())
}
