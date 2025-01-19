#![feature(pattern)]
#![feature(string_remove_matches)]

mod call_server;
mod check_ask;
mod check_setup;
mod generate_context;
mod parse_args;
mod prompt;
mod reset;
mod setup_script;
mod utils;
mod what_to_do;

use anyhow::Result;
use clap::Parser;
use parse_args::ParseArgs;
use std::sync::Mutex;
use utils::{Port, PortTrait, State};
use what_to_do::{
    WhatToDoAfterCheckSetup, WhatToDoAfterParseArgs, WhatToDoAfterParseArgsInternalAction,
};

#[tokio::main]
async fn main() -> Result<()> {
    // converts tracing records to stdout logs in debug mode
    #[cfg(debug_assertions)]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let parse_args = ParseArgs::parse();
    let port = Port::new_mutex();

    run(parse_args, &port).await?;

    let std_out: String = port.to_stdout_format().into();
    let lines = std_out.lines().collect::<Vec<&str>>();
    for line in lines {
        println!("{}", line);
    }
    Ok(())
}

async fn run(args: ParseArgs, port: &Mutex<Port>) -> Result<()> {
    let what_to_do = args.next(port).await?;
    match what_to_do {
        WhatToDoAfterParseArgs::Reset(reset) => {
            reset.next(port).await?;
        }
        WhatToDoAfterParseArgs::PrintVersion {
            cli_version,
            setup_version,
        } => {
            port.log(format!("hey-cli {cli_version}"));
            if let Some(setup_version) = setup_version {
                port.log(format!("setup-script {setup_version}"));
            }
        }
        // TODO: test this branch
        WhatToDoAfterParseArgs::Internal {
            input,
            action: what_to_do,
        } => match what_to_do {
            WhatToDoAfterParseArgsInternalAction::GetStdout => {
                // TODO: move `hey-cli-prompt-start` to a constant
                let everything_before_prompt = match input.contains("hey-cli-prompt-start") {
                    true => input.split("hey-cli-prompt-start").next().unwrap(),
                    false => &input,
                }
                .trim();

                port.log(everything_before_prompt);
            }
            WhatToDoAfterParseArgsInternalAction::GetPrompt => {
                let everything_after_prompt = match input.contains("hey-cli-prompt-start") {
                    true => input.split("hey-cli-prompt-start").last().unwrap().trim(),
                    false => "",
                };
                port.log(everything_after_prompt);
            }
        },
        WhatToDoAfterParseArgs::CheckSetup(check_setup) => {
            let what_to_do = check_setup.next(port).await?;
            match what_to_do {
                WhatToDoAfterCheckSetup::SetupScript(setup_script) => {
                    setup_script.next(port).await?;
                }
                WhatToDoAfterCheckSetup::CheckAsk(check_ask) => {
                    let generate_context = check_ask.next(port).await?;
                    let call_server = generate_context.next(port).await?;
                    let prompt = call_server.next(port).await?;
                    prompt.next(port).await?;
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod end_to_end_tests {
    use crate::{
        parse_args::ParseArgs,
        run,
        utils::{Port, PortTrait},
    };

    const VERSION: &str = env!("CARGO_PKG_VERSION");
    // TODO: make sure to check stdout on all tests

    #[tokio::test]
    async fn version_flag() {
        let port = Port::new_mutex();
        let res = run(
            ParseArgs {
                version: true,
                ..ParseArgs::default()
            },
            &port,
        )
        .await;
        assert!(res.is_ok());
        let stdout = port.to_stdout_format();
        assert_eq!(stdout.into(), format!("hey-cli {VERSION}"));
    }

    #[tokio::test]
    async fn version_flag_shell_name() {
        let port = Port::new_mutex();
        let res = run(
            ParseArgs {
                version: true,
                shell_name: Some("fish".to_string()),
                ..Default::default()
            },
            &port,
        )
        .await;
        assert!(res.is_ok());
        let stdout = port.to_stdout_format();
        assert_eq!(stdout.into(), format!("hey-cli {VERSION}"));
    }

    #[tokio::test]
    async fn version_flag_setup_version() {
        let port = Port::new_mutex();
        let res = run(
            ParseArgs {
                version: true,
                setup_version: Some("1.0.0".to_string()),
                ..Default::default()
            },
            &port,
        )
        .await;
        assert!(res.is_ok());
        let stdout = port.to_stdout_format();
        assert_eq!(stdout.into(), format!("hey-cli {VERSION}"));
    }

    #[tokio::test]
    async fn version_flag_shell_name_setup_version() {
        let port = Port::new_mutex();
        let res = run(
            ParseArgs {
                version: true,
                shell_name: Some("fish".to_string()),
                setup_version: Some("1.0.0".to_string()),
                ..Default::default()
            },
            &port,
        )
        .await;
        assert!(res.is_ok());
        let stdout = port.to_stdout_format();
        assert_eq!(
            stdout.into(),
            format!("hey-cli {VERSION}\nsetup-script fish@1.0.0")
        );
    }

    #[tokio::test]
    async fn ask_no_shell_no_supported_shell_listed() {
        let port = Port::new_mutex();
        let res = run(
            ParseArgs {
                ask: vec![
                    "print".to_string(),
                    "working".to_string(),
                    "directory".to_string(),
                ],
                ..Default::default()
            },
            &port,
        )
        .await;
        assert!(res.is_ok());
        let stdout = port.to_stdout_format();
        assert_eq!(
            stdout.into(),
            r#"Setup script not installed
Installing setup script for shell: fish
Installing setup script for shell: bash
bash shell is not yet supported
Installing setup script for shell: zsh
Installing setup script for shell: power_shell
power_shell shell is not yet supported
Setup script installed successfully
Please open new terminal session"#,
        );
    }

    #[tokio::test]
    async fn ask_no_shell_inside_fish_and_zsh_shells() {
        let port = Port::new_mutex();
        let res = run(
            ParseArgs {
                ask: vec![
                    "print".to_string(),
                    "working".to_string(),
                    "directory".to_string(),
                ],
                ..Default::default()
            },
            &port,
        )
        .await;

        assert!(res.is_ok());
        let stdout = port.to_stdout_format();
        assert_eq!(
            stdout.into(),
            r#"Setup script not installed
Installing setup script for shell: fish
Installing setup script for shell: bash
bash shell is not yet supported
Installing setup script for shell: zsh
Installing setup script for shell: power_shell
power_shell shell is not yet supported
Setup script installed successfully
Please open new terminal session"#
        );
    }

    #[tokio::test]
    async fn ask_with_shell_with_different_setup_version() {
        let port = Port::new_mutex();
        let res = run(
            ParseArgs {
                shell_name: Some("fish".to_string()),
                setup_version: Some("some-different-version".to_string()),
                ask: vec![
                    "print".to_string(),
                    "working".to_string(),
                    "directory".to_string(),
                ],
                ..Default::default()
            },
            &port,
        )
        .await;

        assert!(res.is_ok());
        let stdout = port.to_stdout_format();
        assert_eq!(
            stdout.into(),
            r#"Setup script outdated
Installing setup script for shell: fish
Installing setup script for shell: bash
bash shell is not yet supported
Installing setup script for shell: zsh
Installing setup script for shell: power_shell
power_shell shell is not yet supported
Setup script installed successfully
Please open new terminal session"#
        );
    }

    #[tokio::test]
    async fn ask_with_shell_invalid_ask() {
        let long_ask = "fish ".repeat(21);
        let invalid_asks = vec![
            (
                "print\nworking\ndirectory",
                "Invalid ask: new line character is not allowed",
            ),
            (
                long_ask.trim(),
                "Invalid ask: max length of 100 characters reached",
            ),
        ];
        for (ask, error_message) in invalid_asks {
            let port = Port::new_mutex();
            let res = run(
                ParseArgs {
                    shell_name: Some("fish".to_string()),
                    setup_version: Some("0.1.0".to_string()),
                    ask: ask.split(" ").map(|s| s.to_string()).collect(),
                    ..Default::default()
                },
                &port,
            )
            .await;

            assert!(res.is_err());
            let error = res.unwrap_err();
            assert_eq!(error.to_string(), error_message);
        }
    }

    #[tokio::test]
    async fn ask_with_shell() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .init();

        let port = Port::new_mutex();
        let res = run(
            ParseArgs {
                shell_name: Some("fish".to_string()),
                setup_version: Some("0.1.0".to_string()),
                ask: vec![
                    "print".to_string(),
                    "working".to_string(),
                    "directory".to_string(),
                ],
                ..Default::default()
            },
            &port,
        )
        .await;

        assert!(res.is_ok());
        let stdout = port.to_stdout_format();
        assert_eq!(
            stdout.into(),
            format!("\nhey-cli-prompt-start\necho \"print working directory\"")
        );
    }
}
