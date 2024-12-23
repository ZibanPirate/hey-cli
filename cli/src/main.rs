use clap::Parser;

/// Ask your CLI, next command will be auto-generated.
#[derive(Parser, Debug)]
#[command( about, long_about = None)]
struct Args {
    /// Print version information
    #[arg(short, long)]
    version: bool,
}

fn main() {
    let args = Args::parse();

    if args.version {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        println!("Version: {}", VERSION);
    }
}
