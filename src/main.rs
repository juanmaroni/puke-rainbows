use std::env;
use anyhow::{Result, bail};
use puke_rainbows::{print_colored, get_file_content, convert_to_colored, generate_ansi_file};

const DEFAULT_FILE: &str = "lorem-ipsum";
const HELP_TEXT: &str = 
"Puke Rainbows (puke-rainbows)

Usage:
\tpuke-rainbows -help
\tpuke-rainbows -text <text> [-save]
\tpuke-rainbows -file <file> [-save]

Options:
\t-help \tShow this screen.
\t-text \tPrint a given text.
\t-file \tPrint a given file.
\t-save \tSave output as a file named \"puke.txt\".

Examples:
\tpuke-rainbows -help
\tpuke-rainbows -text Example
\tpuke-rainbows -text \"Hello there, this is an example text\" -save
\tpuke-rainbows -file lorem-ipsum
\tpuke-rainbows -file /path/to/myfile -save";

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let num_args = args.len();
    
    if num_args == 0 {
        // Default
        print_colored(&get_file_content(DEFAULT_FILE)?);
    } else if num_args == 1 {
        if args[0] == "-help" {
            // Print help message
            print_colored(HELP_TEXT);
        } else {
            // Error
            bail!("Wrong argument!\n\n{}\n", HELP_TEXT);
        }
    } else if num_args == 2 {
        if args[0] == "-file" {
            // Read and print file
            print_colored(&get_file_content(&args[1])?);
        } else if args[0] == "-text" {
            // Print text
            print_colored(&args[1]);
        } else {
            // Error
            bail!("Wrong argument!\n\n{}\n", HELP_TEXT);
        } 
    } else if num_args == 3 {
        if args[2] != "-save" {
            bail!("Wrong argument!\n\n{}\n", HELP_TEXT);
        }

        if args[0] == "-file" {
            // Read and print file
            let output = &get_file_content(&args[1])?;
            print_colored(&output);
            generate_ansi_file(&convert_to_colored(&output))?;
        } else if args[0] == "-text" {
            // Print text
            let output = &args[1];
            print_colored(&output);
            generate_ansi_file(&convert_to_colored(&output))?;
        } else {
            // Error
            bail!("Wrong argument!\n\n{}\n", HELP_TEXT);
        }
    } else {
        // Error
        bail!("Wrong number of arguments!\n\n{}\n", HELP_TEXT);
    }

    Ok(())
}
