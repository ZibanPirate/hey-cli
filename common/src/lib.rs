use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CliPrompt {
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCliPromptResponse {
    pub prompt: CliPrompt,
}

#[derive(Deserialize, Debug)]
pub struct GetCliPromptRequestQuery {
    pub q: String,
}
