use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct CliPrompt {
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCliPromptResponse {
    pub prompt: CliPrompt,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCliPromptRequestBody {
    pub ask: String,
    pub context: HashMap<String, HashMap<String, String>>,
}
