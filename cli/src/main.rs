use clap::Parser;
use hey_cli_common::GetCliPromptResponse;
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

    /// Your ask
    #[arg()]
    ask: Vec<String>,
}

#[tokio::main]
async fn main() {
    // converts tracing records to stdout logs in debug mode
    #[cfg(debug_assertions)]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let args = Args::parse();

    if args.version {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        println!("hey {}", VERSION);
        return;
    }

    if args.shell.is_none() {
        println!("Setting up hey in your shells");
        let setup = include_str!("../../scripts/setup_hey_cli.fish").to_string();
        let fish_setup_relative_path = ".config/fish/functions/setup_hey_cli.fish";
        let home_dir = dirs::home_dir().expect("Could not find home directory");
        let fish_setup_path = home_dir.join(fish_setup_relative_path);
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

        let config_source_line = format!("source ~/{}", fish_setup_relative_path);
        if !config_content.contains(&config_source_line) {
            if !config_content.ends_with("\n") {
                file.write_all(b"\n").expect("Could not write to file");
            }
            file.write_all(config_source_line.as_bytes())
                .expect("Could not write to file");
        }

        println!("upserted file: {}", fish_setup_path.to_str().unwrap());
        println!("Please restart your shell to see the changes");
        return;
    }

    let ask = args.ask.join(" ");
    tracing::info!("Prompt: {ask}");

    #[cfg(not(debug_assertions))]
    let server_url = "http://134.209.220.76";
    #[cfg(debug_assertions)]
    let server_url = "http://0.0.0.0:3000";

    #[cfg(debug_assertions)]
    {
        // wait on /health and retry every 1 seconds
        let url = format!("{server_url}/health");
        loop {
            let resp = reqwest::get(&url).await;
            match resp {
                Ok(_) => {
                    tracing::info!("Server is up.");
                    break;
                }
                Err(e) => {
                    tracing::warn!("Server is not up yet: {e}");
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
        }
    }

    let url = format!("{server_url}/cli-prompt?q={ask}");
    let resp = reqwest::get(url).await.expect("Could not get response");
    let resp = resp
        .json::<GetCliPromptResponse>()
        .await
        .expect("Could not get response");
    tracing::debug!("repo: {resp:#?}");

    let output_prompt = resp.prompt.value;

    tracing::info!("Ready.");
    println!("{output_prompt}");
}
