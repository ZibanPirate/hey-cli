use crate::{
    check_ask::CheckAsk, check_setup::CheckSetup, reset::Reset, setup_script::SetupScript,
};
use nest_struct::nest_struct;

#[nest_struct]
pub enum WhatToDoAfterParseArgs {
    Reset(Reset),
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
