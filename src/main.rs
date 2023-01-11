use std::{env, fs};
use anyhow::{Context, Result, anyhow};
use puke_rainbows::print_colored;

const DEFAULT_FILE: &str = "lorem-ipsum";
const HELP_TEXT: &str = "Usage: puke-rainbows {{-h | -t <text> | -f <file>}}";

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let num_args = args.len();
    
    if num_args == 0 {
        // Default
        print_file_content(DEFAULT_FILE)?;
    } else if num_args == 1 {
        if args[0] == "-h" {
            // Print help message
            print_colored(String::from(HELP_TEXT));
        } else {
            // Error
            return Err(anyhow!("Wrong argument!"));
        }
    } else if num_args == 2 {
        if args[0] == "-f" {
            // Read and print file
            print_file_content(&args[1])?;
        } else if args[0] == "-t" {
            // Print text
            print_colored(args[1].to_owned());
        } else {
            // Error
            return Err(anyhow!("Wrong argument!"));
        }
    } else {
        // Error
        return Err(anyhow!("Wrong number of arguments!"));
    }

    Ok(())
}

fn print_file_content(filepath: &str) -> Result<()> {
    let f = fs::read_to_string(filepath)
        .with_context(|| format!("could not read file `{}`", filepath))?;
    print_colored(f);

    Ok(())
}
