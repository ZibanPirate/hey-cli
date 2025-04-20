#!/usr/bin/env RUST_BACKTRACE=1 cargo +nightly -Zscript

---
package.edition = "2024"

[dependencies]
cli-run = { git = "https://github.com/zibanpirate/cli-rs.git" }
---

fn main() {
    println!("Ensuring cargo binstall is installed ...");
    cli_run::cli_run("cargo", vec!["install", "cargo-binstall"]);

    println!("Ensuring cross is binstalled ...");
    cli_run::cli_run("cargo", vec!["binstall", "cross", "-y"]);

    println!("Ensuring docker is running ...");
    cli_run::cli_run("docker", vec!["ps"]);

    println!("Building ./hey-cli-server for ubuntu x86_64 ...");
    cli_run::cli_run(
        "cross",
        vec![
            "build",
            "-p",
            "hey-cli-server",
            "--profile",
            "release-server",
            "--target",
            "x86_64-unknown-linux-gnu",
        ],
    );

    println!("Building docker image ...");
    cli_run::cli_run(
        "docker",
        vec![
            "build",
            ".",
            "-t",
            "ghcr.io/zibanpirate/hey-cli-server:latest",
        ],
    );

    println!("Logging in to GitHub Container Registry ...");
    let gh_token = std::env::var("DOCKER_REGISTRY_PASSWORD")
        .expect("DOCKER_REGISTRY_PASSWORD environment variable not set");
    cli_run::cli_run(
        "docker",
        vec![
            "login",
            "ghcr.io",
            "-u",
            "zibanpirate",
            "--password",
            &gh_token,
        ],
    );

    println!("Pushing docker image ...");
    cli_run::cli_run(
        "docker",
        vec!["push", "ghcr.io/zibanpirate/hey-cli-server:latest"],
    );

    println!("Deploying to zcluster ...");
    cli_run::cli_run(
        "zcluster",
        vec!["deploy", "-p", "hey-cli", "./docker-compose.yml"],
    );
    println!("Done!");
}
