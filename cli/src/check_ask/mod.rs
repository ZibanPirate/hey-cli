use crate::{
    generate_context::GenerateContext,
    utils::{PortTrait, State},
};
use anyhow::Result;

pub struct CheckAsk {
    pub ask: String,
}

const MAX_ASK_LENGTH: usize = 100;

impl State<GenerateContext> for CheckAsk {
    async fn next(self, _: &impl PortTrait) -> Result<GenerateContext> {
        if self.ask.contains("\n") {
            return Err(anyhow::anyhow!(
                "Invalid ask: new line character is not allowed"
            ));
        }

        if self.ask.len() > MAX_ASK_LENGTH {
            return Err(anyhow::anyhow!(
                "Invalid ask: max length of {} characters reached",
                MAX_ASK_LENGTH
            ));
        }

        Ok(GenerateContext::new(self.ask))
    }
}
