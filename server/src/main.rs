use async_openai::{
    Client,
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestSystemMessage, ChatCompletionRequestUserMessage,
        CreateChatCompletionRequestArgs,
    },
};
use axum::{
    Json, Router,
    routing::{get, post},
};
use dotenv::dotenv;
use hey_cli_common::{CliPrompt, GetCliPromptRequestBody, GetCliPromptResponse};
use std::collections::HashMap;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let ts = tracing_subscriber::registry().with(tracing_subscriber::fmt::layer());

    ts.init();

    let app = Router::new()
        .route("/", get(root))
        .route("/cli-prompt", post(post_cli_prompt))
        .route("/health", get(health))
        .route("/install.sh", get(get_install_script));

    let fallback_port = "3000";
    let port = std::env::var("PORT").unwrap_or(fallback_port.to_string());
    let bind = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&bind).await.unwrap();

    tracing::info!("Server: http://{}", bind);

    axum::serve(listener, app).await.unwrap();
}

#[tracing::instrument]
async fn post_cli_prompt(query: Json<GetCliPromptRequestBody>) -> Json<GetCliPromptResponse> {
    let prompt = generate_cli_prompt(&query.ask, &query.context).await;

    Json(GetCliPromptResponse { prompt })
}

// TODO: add anyhow error handling
#[tracing::instrument(ret)]
async fn generate_cli_prompt(
    user_message: &String,
    context: &HashMap<String, HashMap<String, String>>,
) -> CliPrompt {
    let openai_key = std::env::var("OPENAI_KEY").unwrap();
    let openai_organization_id = std::env::var("OPENAI_ORGANIZATION_ID").unwrap();

    // TODO: save the client in a global state
    let config = OpenAIConfig::new()
        .with_api_key(openai_key)
        .with_org_id(openai_organization_id);

    let client = Client::with_config(config);

    let messages = [
        ChatCompletionRequestSystemMessage::from(
            "The user will give you some context in form of JSON, then right after, the user will ask a question, and your job is to model the answer in a command line interface.",
        )
        .into(),
        ChatCompletionRequestSystemMessage::from(
            "Your response must be a one-liner valid command that can be run in a shell. no extra, no code blocks.",
        )
        .into(),
        ChatCompletionRequestSystemMessage::from(
            "In the case where you don't have an answer, you can respond with `echo \"[your excuse]\"`",
        )
        .into(),
        ChatCompletionRequestUserMessage::from(format!(r#"user context:
```json
{}
```
"#, serde_json::to_string(context).expect("Failed to serialize context"))).into(),
        ChatCompletionRequestUserMessage::from(format!(r#"user ask:
{}
"#, user_message.clone())).into(),
    ];

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u32)
        .model("gpt-4o-2024-08-06")
        .messages(messages)
        .build()
        .expect("Failed to build request");

    let response = client
        .chat()
        .create(request)
        .await
        .expect("Failed to create chat completion");

    let value = response.choices.first().unwrap();
    let value = &value.message.content;
    let value = value.clone().unwrap();

    CliPrompt { value }
}

#[tracing::instrument]
async fn health() -> &'static str {
    tracing::info!("Health check: OK");
    "OK"
}

#[tracing::instrument]
async fn get_install_script() -> &'static str {
    let content = include_str!("scripts/install.sh");
    content
}

const HTML: &str = include_str!("./home.html");

#[tracing::instrument]
async fn root() -> axum::response::Html<&'static str> {
    axum::response::Html(HTML)
}
