use clap::Parser;
use std::fs::File;
use std::io::Read;
use thiserror::Error;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// The path of the file or directory of which the lines should be counted
    file_path: String,

    /// Skip empty lines
    #[clap(short, takes_value = false)]
    skip_empty_lines: bool,

    /// Enable the recursive flag.
    /// line_counter will count lines in subdirectories recursively
    #[clap(short, long, takes_value = false)]
    recursive: bool,

    #[clap(short, long, takes_value = false)]
    count_chars: bool,

    #[clap(short, long, takes_value = false)]
    words: bool,

    /// To ignore files completely add a ".ignore.lc" file to the directory and write down the files that should be ignored.
    ignored: Vec<String>,
}

impl Args {
    /// Checks if a ".ignore.lc" file is within the directory, and adds them to the ignored_vec.
    fn with_ignored(mut self) -> Result<Self> {
        if !std::fs::metadata(&self.file_path)?.is_dir() {
            return Ok(self);
        }
        for entry in std::fs::read_dir(&self.file_path)?.flatten() {
            if entry.file_name() == ".lcignore" {
                let mut f = File::open(entry.path())?;

                let mut ignored = String::new();
                f.read_to_string(&mut ignored)?;

                self.ignored = ignored
                    .lines()
                    .map(|line| line.trim().to_string())
                    .collect();
                self.ignored.push(String::from(".lcignore"));
            }
        }
        Ok(self)
    }
}

#[derive(Debug, Error)]
enum Error {
    #[error("Error occurred while reading file")]
    LcIoError(#[from] std::io::Error),
}

type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = Args::parse().with_ignored()?;

    let file_metadata = std::fs::metadata(&args.file_path)?;

    if file_metadata.is_dir() {
        if let Some(d_data) = get_dir_data(&args.file_path, &args)? {
            print_dir(&d_data, &args);
            println!("Total lines: {total}", total = d_data.total_lines());
            println!("Total characters: {total}", total = d_data.total_characters());
            println!("words: {total}", total = d_data.total_words());
        }
    } else {
        let f_data = get_file_data(&args.file_path, args.skip_empty_lines)?;
        print_file(&f_data, &args);
    }

    Ok(())
}

fn print_file(file: &FileData, args: &Args) {
    println!(
        "{file_name} => {line_count} lines {chars} {word}",
        // word = &file.words,
        word = if args.words {
            format!("and {} Words", &file.words)
        } else {
            "".to_owned()
        },
        file_name = &file.file_name,
        line_count = file.lines,
        chars = if args.count_chars {
            format!("({chars} chars)", chars = file.characters)
        } else {
            "".to_owned()
        }
    );
}

fn print_dir(dir: &DirData, args: &Args) {
    println!("{dir_name}: ", dir_name = &dir.dir_name);
    for file in &dir.file_data {
        print!("\t");
        print_file(file, args);
    }
    for dir in &dir.sub_dirs {
        print!("\t\t");
        print_dir(dir, args);
    }
}

struct FileData {
    file_name: String,
    lines: usize,
    characters: usize,
    words: usize,
}

struct DirData {
    dir_name: String,
    file_data: Vec<FileData>,
    sub_dirs: Vec<DirData>,
}

impl DirData {
    fn total_lines(&self) -> usize {
        let mut total = 0;
        for f in &self.file_data {
            total += f.lines;
        }
        total
    }

    fn total_characters(&self) -> usize {
        let mut total = 0;
        for f in &self.file_data {
            total += f.characters;
        }
        total
    }

    fn total_words(&self) -> usize {
        let mut total = 0;
        for f in &self.file_data {
            total += f.words;
        }
        total
    }
}

fn get_file_data(path: impl Into<String>, skip_empty_lines: bool) -> Result<FileData> {
    let file_name: String = path.into();

    let mut f = File::open(&file_name)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    let mut lines = 0;
    let mut characters = 0;
    let mut words = 0;
    let mut empty_lines = 0;

    for line in s.lines() {
        if !line.trim().is_empty() {
            lines += 1;
        } else {
            empty_lines += 1;
        }
    }
    if skip_empty_lines {
        lines = s.lines().count() - empty_lines
    } else {
        lines = s.lines().count();
    }

    for char in s.chars() {
        if char != '\n' || char != '\t' {
            characters += 1;
            if char.is_whitespace() {
                words += 1;
            }
        }
    }

    let words = words - empty_lines;

    Ok(FileData {
        file_name,
        lines,
        characters,
        words,
    })
}

fn get_dir_data(dir_path: &str, args: &Args) -> Result<Option<DirData>> {
    let mut dir_data = DirData {
        dir_name: dir_path.to_owned(),
        file_data: vec![],
        sub_dirs: vec![],
    };
    if args.ignored.contains(&dir_path.to_owned()) {
        return Ok(None);
    }
    for entry in std::fs::read_dir(dir_path).into_iter().flatten() {
        let e = if entry.is_ok() {
            entry.unwrap()
        } else {
            continue;
        };
        if args.recursive && e.metadata()?.is_dir() {
            if let Some(data) = get_dir_data(e.path().to_str().unwrap(), args)? {
                dir_data.sub_dirs.push(data);
            }
            continue;
        }
        if e.metadata()?.is_file() {
            dir_data.file_data.push(get_file_data(
                e.path().to_str().unwrap(),
                args.skip_empty_lines,
            )?);
        }
    }
    Ok(Some(dir_data))
}
