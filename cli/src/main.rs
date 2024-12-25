use clap::Parser;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

/// Ask your CLI, next command will be auto-generated.
#[derive(Parser, Debug)]
#[command( about, long_about = None)]
struct Args {
    /// Print version information
    #[arg(short, long)]
    version: bool,

    #[arg(long)]
    /// Which shell to use
    shell: Option<String>,

    /// Prompt to ask
    #[arg()]
    prompt: Vec<String>,
}

fn main() {
    let args = Args::parse();

    if args.version {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        println!("hey {}", VERSION);
        return;
    }

    if args.shell.is_none() {
        println!("Setting up hey in your shells");
        let setup = include_str!("../../scripts/setup_hey.fish").to_string();
        let home_dir = dirs::home_dir().expect("Could not find home directory");
        let fish_setup_path = home_dir.join(".config/fish/functions/setup_hey.fish");
        fs::create_dir_all(fish_setup_path.parent().unwrap())
            .expect("Could not create directories");
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&fish_setup_path)
            .expect("Could not open file");
        file.write_all(setup.as_bytes())
            .expect("Could not write to file");

        let fish_config_path = home_dir.join(".config/fish/config.fish");
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&fish_config_path)
            .expect("Could not open file");

        let config_content = fs::read_to_string(&fish_config_path).expect("Could not read file");

        if !config_content.contains("source ~/.config/fish/functions/setup_hey.fish") {
            file.write_all(b"\nsource ~/.config/fish/functions/setup_hey.fish\n")
                .expect("Could not write to file");
        }

        println!("upserted file: {}", fish_setup_path.to_str().unwrap());
        println!("Please restart your shell to see the changes");
        return;
    }

    println!("echo \"{}\"", args.prompt.join(" "));
}
