use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[clap(
        short = 'd',
        long = "dir",
        value_name = "DIRECTORY",
        help = "Sets the working directory"
    )]
    pub dir: String,

    #[clap(
        short = 'r',
        long = "recursive",
        help = "Enables recursive directory processing"
    )]
    pub recursive: bool,
    // TODO: Add log level flag
}
