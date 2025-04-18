use crate::{
    prompt::Prompt,
    utils::{PortTrait, State},
};
use anyhow::Result;
use hey_cli_common::GetCliPromptRequestBody;
use std::collections::HashMap;

pub struct CallServer {
    pub ask: String,
    pub context: HashMap<String, HashMap<String, String>>,
}

impl State<Prompt> for CallServer {
    async fn next(self, port: &impl PortTrait) -> Result<Prompt> {
        let query = GetCliPromptRequestBody {
            ask: self.ask,
            context: self.context,
        };

        let prompt = port.ask_server_for_prompt(query).await?;

        // TODO: check and print update notice

        Ok(Prompt {
            value: prompt.prompt.value,
        })
    }
}
