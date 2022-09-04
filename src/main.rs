use std::{
    fs::{DirEntry, File},
    io::Read,
};

use clap::Parser;
use thiserror::Error;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// The path of the file or directory of which the lines should be counted
    file_name: String,

    /// Skip empty lines
    #[clap(short, takes_value=false)]
    skip_empty_lines: bool,
}

#[derive(Debug, Error)]
enum Error {
    #[error("Error occurred while reading file")]
    LcIoError(#[from] std::io::Error),

    #[error("Error occurred while parsing file name")]
    FileNameError,
}

type Result<T> = std::result::Result<T, Error>;
fn main() -> Result<()> {
    let mut lines: usize = 0;

    let args = Args::parse();
    let file_name = args.file_name;

    let is_dir = std::fs::metadata(&file_name)?.is_dir();

    if is_dir {
        lines = get_dir_lines(&file_name, args.skip_empty_lines, 0)?;
    } else {
        let mut file = File::open(&file_name)?;
        let mut buffer = String::new();

        file.read_to_string(&mut buffer)?;

        if args.skip_empty_lines {
            for line in buffer.lines() {
                if line.trim().is_empty() {
                    lines += 1;
                }
            }
        } else {
            lines += buffer.lines().count();
        }
    }

    println!(
        "\nTotal number of lines in {}: {}",
        if is_dir { "directory" } else { "file" },
        lines
    );

    Ok(())
}

fn get_dir_lines(file_name: &str, skip_empty_lines: bool, depth: usize) -> Result<usize> {
    let mut lines = 0;
    let mut indenting = String::new();

    let mut maybe_dirs: Vec<DirEntry> = Vec::new();

    for _d in 0..depth {
        indenting += "  ";
    }

    println!("{}{}:", indenting, file_name);
    for entry in std::fs::read_dir(&file_name)? {
        let entry = entry?;

        if entry.metadata()?.is_dir() {
            maybe_dirs.push(entry);
            continue;
        }
        
        let mut file = File::open(entry.path())?;
        
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let mut current_lines = 0_usize;
        
        if skip_empty_lines {
            for line in buffer.lines() {
                if !line.trim().is_empty() {
                    current_lines += 1;
                }
            }
        } else {
            current_lines += buffer.lines().count();
        }

        println!(
            "{}> {}: {}",
            indenting,
            entry.file_name().to_str().ok_or(Error::FileNameError)?,
            current_lines
        );
        if current_lines == 69 {
            println!("  NICE!");
        }
        lines += current_lines;
    }
    for dir in maybe_dirs {
        lines += get_dir_lines(
            dir.path().to_str().ok_or(Error::FileNameError)?,
            skip_empty_lines,
            depth + 1,
        )?;
    }

    Ok(lines)
}
