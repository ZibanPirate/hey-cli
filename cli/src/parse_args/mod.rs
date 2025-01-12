// TODO: lint against using print! and println! macros

use crate::{
    check_setup::CheckSetup,
    reset::Reset,
    utils::{PortTrait, Shell, ShellName, State},
    what_to_do::{WhatToDoAfterParseArgs, WhatToDoAfterParseArgsInternalAction},
};
use anyhow::Result;
use clap::Parser;
use std::str::FromStr;

/// Ask your CLI, next command will be auto-generated.
#[derive(Parser, Debug, Default)]
#[command( about, long_about = None)]
pub struct ParseArgs {
    /// Print version information
    #[arg(short, long)]
    pub version: bool,

    /// Reset the setup
    #[arg(short, long)]
    pub reset: bool,

    /// Internal: Get the stdout for setup script
    #[arg(long)]
    pub get_stdout: bool,

    /// Internal: Get the prompt for setup script
    #[arg(long)]
    pub get_prompt: bool,

    #[arg(long)]
    /// Which shell to use
    pub shell_name: Option<String>,

    #[arg(long)]
    /// The version of the setup script
    pub setup_version: Option<String>,

    /// Your ask
    #[arg()]
    pub ask: Vec<String>,
}

impl State<WhatToDoAfterParseArgs> for ParseArgs {
    async fn next(self, _: &impl PortTrait) -> Result<WhatToDoAfterParseArgs> {
        let shell = match (self.shell_name, self.setup_version) {
            (Some(shell_name), Some(setup_version)) => Some(Shell {
                setup_version,
                name: ShellName::from_str(&shell_name)?,
            }),
            _ => None,
        };

        if self.reset {
            return Ok(WhatToDoAfterParseArgs::Reset(Reset));
        }

        if self.version {
            return Ok(WhatToDoAfterParseArgs::PrintVersion {
                cli_version: env!("CARGO_PKG_VERSION").to_string(),
                setup_version: match shell {
                    Some(shell) => Some(format!("{}@{}", shell.name, shell.setup_version)),
                    _ => None,
                },
            });
        }

        let ask = self.ask.join(" ");
        if self.get_stdout {
            return Ok(WhatToDoAfterParseArgs::Internal {
                input: ask,
                action: WhatToDoAfterParseArgsInternalAction::GetStdout,
            });
        }

        if self.get_prompt {
            return Ok(WhatToDoAfterParseArgs::Internal {
                input: ask,
                action: WhatToDoAfterParseArgsInternalAction::GetPrompt,
            });
        }

        Ok(WhatToDoAfterParseArgs::CheckSetup(CheckSetup {
            shell,
            ask,
        }))
    }
}
