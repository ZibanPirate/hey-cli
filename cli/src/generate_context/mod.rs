use crate::{
    call_server::CallServer,
    utils::{PortTrait, State},
};
use anyhow::Result;
use std::collections::HashMap;

pub struct GenerateContext {
    pub ask: String,
}

impl GenerateContext {
    pub fn new(ask: String) -> Self {
        Self { ask }
    }
}

impl State<CallServer> for GenerateContext {
    async fn next(self, _: &impl PortTrait) -> Result<CallServer> {
        let mut general_context = HashMap::new();
        let info = os_info::get();

        general_context.insert("os_type".to_string(), info.os_type().to_string());
        general_context.insert("os_version".to_string(), info.version().to_string());
        general_context.insert("os_bitness".to_string(), info.bitness().to_string());
        if let Some(arch) = info.architecture() {
            general_context.insert("os_architecture".to_string(), arch.to_string());
        }

        // TODO: generate more context from plugins

        let mut context = HashMap::new();
        context.insert("general".to_string(), general_context);

        Ok(CallServer {
            ask: self.ask,
            context,
        })
    }
}
