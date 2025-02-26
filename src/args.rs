use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "fcat",
    version,
    about = "Fast command-line file concatenator with smart filtering, recursive scanning, and clipboard integration",
    long_about = None
)]
pub struct CliArgs {
    /// Paths to files or directories (shell glob expansion is supported)
    #[arg(required = true)]
    pub paths: Vec<String>,

    /// Save output to file (default: paste.txt when flag is used without value)
    #[arg(short, long, value_name = "FILE", default_missing_value("paste.txt"))]
    pub output: Option<String>,

    /// Ignore patterns. All arguments following '-i' will be considered ignore patterns.
    /// (Make sure to list positional paths before the '-i' flag.)
    #[arg(short, long, value_name = "PATTERN", num_args = 1.., last = true)]
    pub ignore: Vec<String>,
}
