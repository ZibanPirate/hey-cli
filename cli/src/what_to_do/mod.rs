use nest_struct::nest_struct;

use crate::{check_ask::CheckAsk, check_setup::CheckSetup, setup_script::SetupScript};

#[nest_struct]
pub enum WhatToDoAfterParseArgs {
    PrintVersion {
        cli_version: String,
        setup_version: Option<String>,
    },
    Internal {
        input: String,
        action: nest! {
            GetStdout,
            GetPrompt,
        },
    },
    CheckSetup(CheckSetup),
}

pub enum WhatToDoAfterCheckSetup {
    SetupScript(SetupScript),
    CheckAsk(CheckAsk),
}
