mod cli;
mod file_metadata;
mod file_processor;
mod logger;

use std::path::Path;

use clap::Parser;
use cli::Cli;
use file_processor::directory::process_dir;
use logger::{fatal, info};

fn main() {
    let args = Cli::parse();
    let dir_path = Path::new(&args.dir);

    if !dir_path.is_dir() {
        fatal(format!("{} is not a directory", args.dir).as_str());
        std::process::exit(1);
    }

    info("Starting file date fix process");

    process_dir(dir_path, args.recursive);
}
