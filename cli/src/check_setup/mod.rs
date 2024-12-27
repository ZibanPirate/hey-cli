use crate::{
    check_ask::CheckAsk,
    setup_script::SetupScript,
    utils::{PortTrait, Shell, State},
    what_to_do::WhatToDoAfterCheckSetup,
};
use anyhow::Result;

pub struct CheckSetup {
    pub shell: Option<Shell>,
    pub ask: String,
}

impl State<WhatToDoAfterCheckSetup> for CheckSetup {
    async fn next(self, port: &impl PortTrait) -> Result<WhatToDoAfterCheckSetup> {
        if self.shell.is_none() {
            port.log("Setup script not installed");
            return Ok(WhatToDoAfterCheckSetup::SetupScript(SetupScript(None)));
        }

        let shell = self.shell.unwrap();
        if shell.setup_version != shell.expected_setup_version() {
            port.log("Setup script outdated");
            return Ok(WhatToDoAfterCheckSetup::SetupScript(SetupScript(Some(
                shell.name,
            ))));
        }

        Ok(WhatToDoAfterCheckSetup::CheckAsk(CheckAsk {
            ask: self.ask,
        }))
    }
}
