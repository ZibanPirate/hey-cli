use crate::{
    call_server::CallServer,
    utils::{PortTrait, State},
};
use anyhow::Result;
use std::collections::HashMap;

pub struct GenerateContext {
    pub ask: String,
    pub context: HashMap<String, HashMap<String, String>>,
}

impl GenerateContext {
    pub fn new(ask: String) -> Self {
        Self {
            ask,
            context: HashMap::new(),
        }
    }
}

impl State<CallServer> for GenerateContext {
    async fn next(self, _: &impl PortTrait) -> Result<CallServer> {
        // TODO: generate default context
        // TODO: generate context from plugins
        Ok(CallServer {
            ask: self.ask,
            context: self.context,
        })
    }
}
