use anyhow::Result;

use crate::utils::{PortTrait, State};

pub struct Prompt {
    pub value: String,
}

impl State<()> for Prompt {
    async fn next(self, port: &impl PortTrait) -> Result<()> {
        port.set_final_prompt(self.value);

        Ok(())
    }
}
