use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestSystemMessage, ChatCompletionRequestUserMessage,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use axum::{extract::Query, routing::get, Json, Router};
use common::{CliPrompt, GetCliPromptRequestQuery, GetCliPromptResponse};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    // converts tracing records to stdout logs
    #[cfg(debug_assertions)] // debug mode
    let max_log_level = tracing::Level::DEBUG;
    #[cfg(not(debug_assertions))] // release mode
    let max_log_level = tracing::Level::WARN;
    tracing_subscriber::fmt()
        .with_max_level(max_log_level)
        .init();

    dotenv().ok();

    let app = Router::new()
        .route("/cli-prompt", get(get_cli_prompt))
        .route("/health", get(health));

    let fallback_port = "3000";
    let port = std::env::var("PORT").unwrap_or(fallback_port.to_string());
    let bind = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&bind).await.unwrap();

    tracing::info!("Server: http://{}", bind);

    axum::serve(listener, app).await.unwrap();
}

async fn get_cli_prompt(
    Query(query): Query<GetCliPromptRequestQuery>,
) -> Json<GetCliPromptResponse> {
    let openai_key = std::env::var("OPENAI_KEY").unwrap();
    let openai_organization_id = std::env::var("OPENAI_ORGANIZATION_ID").unwrap();

    let config = OpenAIConfig::new()
        .with_api_key(openai_key)
        .with_org_id(openai_organization_id);

    let client = Client::with_config(config);

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u32)
        .model("gpt-4o-2024-08-06")
        .messages([
            ChatCompletionRequestSystemMessage::from(
                "The user will ask a question, and your job is to model the answer in a command line interface.",
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

            ChatCompletionRequestUserMessage::from(query.q).into(),
        ])
        .build()
        .expect("Failed to build request");

    let response = client
        .chat()
        .create(request)
        .await
        .expect("Failed to create chat completion");

    let value = response.choices.get(0).unwrap();
    let value = &value.message.content;
    let value = value.clone().unwrap();

    let prompt = CliPrompt { value };

    Json(GetCliPromptResponse { prompt })
}

async fn health() -> &'static str {
    tracing::info!("Health check: OK");
    "OK"
}
