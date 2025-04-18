#!/usr/bin/env RUST_BACKTRACE=1 cargo +nightly -Zscript

---
package.edition = "2024"

[dependencies]
cli-run = { git = "https://github.com/zibanpirate/cli-rs.git" }
---

fn main() {
    let cwd = cli_run::get_cli_run_cwd();
    let dot_env_path = std::path::Path::new(&cwd).join(".env");
    if !dot_env_path.exists() {
        eprintln!("Please ensure {} exists", dot_env_path.display());
        std::process::exit(1);
    }
    println!("Installing bacon...");
    cli_run::cli_run("cargo", vec!["install", "bacon"]);
    println!("Installing wait-on...");
    cli_run::cli_run("cargo", vec!["install", "wait-on"]);

    println!("running ./infra in debug mode...");
    // get the args and pass it to bacon
    let args = std::env::args().collect::<Vec<_>>();
    cli_run::cli_run("bacon", args[1..].to_vec());
}
