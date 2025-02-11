use clap::Parser;
use std::fs;
use std::fs::DirEntry;
use std::io::Error;
use std::path::Path;

#[derive(Parser)]
struct Cli {
    #[clap(
        short = 'd',
        long = "dir",
        value_name = "DIRECTORY",
        help = "Sets the working directory"
    )]
    dir: String,

    #[clap(
        short = 'r',
        long = "recursive",
        help = "Enables recursive directory processing"
    )]
    recursive: bool,
}

fn main() {
    let args = Cli::parse();
    let dir_path = Path::new(&args.dir);

    if !dir_path.is_dir() {
        eprintln!("{} is not a directory", args.dir);
        return;
    }

    process_dir(dir_path, args.recursive);
}

fn process_dir(dir: &Path, recursive: bool) {
    let files = fs::read_dir(dir).expect("Unable to read specified directory");

    for entry in files {
        process_dir_entry(entry, recursive);
    }
}

fn process_dir_entry(entry: Result<DirEntry, Error>, recursive: bool) {
    if let Ok(entry) = entry {
        let file_path = entry.path();

        if file_path.is_dir() {
            handle_directory_entry(&file_path, recursive);
        } else {
            handle_file_entry(&file_path);
        }
    }
}

fn handle_directory_entry(dir_path: &Path, recursive: bool) {
    if recursive {
        process_dir(dir_path, recursive);
    } else {
        println!(
            "Skipping directory: {}. Use -r to process directories recursively.",
            dir_path.display()
        );
    }
}

fn handle_file_entry(file_path: &Path) {
    if let Some(file_name) = extract_utf8_file_name(file_path) {
        process_file(file_path, file_name);
    } else {
        eprintln!(
            "Unable to extract file name from path: {}",
            file_path.display()
        );
    }
}

fn extract_utf8_file_name(path: &Path) -> Option<&str> {
    path.file_name()?.to_str()
}

fn process_file(file_path: &Path, file_name: &str) {
    println!("Processing: {}", file_name);
}
