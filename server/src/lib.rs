use axum::{
    extract,
    routing::{get, post},
    Json, Router,
};
use hey_cli_common::{CliPrompt, GetCliPromptRequestBody, GetCliPromptResponse};
use nest_struct::nest_struct;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tower_service::Service;
use worker::*;

#[nest_struct]
#[derive(Clone)]
struct AppState {
    config: nest! {
        openai_key: String,
        openai_organization_id: String,
    },
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();

    let state = AppState {
        config: AppStateConfig {
            openai_key: env
                .secret("OPENAI_KEY")
                .expect("OPENAI_KEY is required")
                .to_string(),
            openai_organization_id: env
                .secret("OPENAI_ORGANIZATION_ID")
                .expect("OPENAI_ORGANIZATION_ID is required")
                .to_string(),
        },
    };

    Ok(Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/cli-prompt", post(post_cli_prompt))
        .route("/install.sh", get(get_install_script))
        .with_state(state)
        .call(req)
        .await?)
}

#[derive(Debug, Serialize, Deserialize)]
struct Que {
    sup: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Res {
    res: String,
}

#[worker::send]
async fn post_cli_prompt(
    extract::State(state): extract::State<AppState>,
    Json(query): Json<GetCliPromptRequestBody>,
) -> Json<GetCliPromptResponse> {
    let prompt = generate_cli_prompt(
        &query.ask,
        &query.context,
        &state.config.openai_key,
        &state.config.openai_organization_id,
    )
    .await;

    Json(GetCliPromptResponse { prompt })
}

#[nest_struct]
#[derive(Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<
        nest! {
            role: String,
            content: String,
        },
    >,
}

#[nest_struct]
#[derive(Deserialize)]
struct OpenAiResponse {
    choices: Vec<nest! { message: nest! { content: String } }>,
}

async fn generate_cli_prompt(
    user_message: &str,
    context: &HashMap<String, HashMap<String, String>>,
    openai_key: &str,
    openai_organization_id: &str,
) -> CliPrompt {
    // TODO: save the client in a global state
    let client = reqwest::Client::new();

    let request = OpenAiRequest {
        model: "gpt-4o-mini".to_string(),
        messages: vec![
            OpenAiRequestMessages {
                role: "system".to_string(),
                content: "The user will give you some context in form of JSON, then right after, the user will ask a question, and your job is to model the answer in a command line interface.".to_string(),
            },
            OpenAiRequestMessages {
                role: "system".to_string(),
                content: "Your response must be a one-liner valid command that can be run in a shell. no extra, no code blocks.".to_string(),
            },
            OpenAiRequestMessages {
                role: "system".to_string(),
                content: "In the case where you don't have an answer, you can respond with `echo \"[your excuse]\"`".to_string(),
            },
            OpenAiRequestMessages {
                role: "user".to_string(),
                content: format!(
                    "user context:\n```json\n{}\n```\n",
                    serde_json::to_string(context).expect("Failed to serialize context")
                ),
            },
            OpenAiRequestMessages {
                role: "user".to_string(),
                content: format!("user ask:\n{}", user_message),
            },
        ],
    };

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", openai_key))
        .header("OpenAI-Organization", openai_organization_id)
        .json(&request)
        .send()
        .await
        .expect("Failed to send request")
        .json::<OpenAiResponse>()
        .await
        .expect("Failed to parse response");

    CliPrompt {
        value: response.choices.first().unwrap().message.content.clone(),
    }
}

const HTML: &str = include_str!("./home.html");

async fn root() -> axum::response::Html<&'static str> {
    axum::response::Html(HTML)
}

async fn health() -> &'static str {
    "OK"
}

async fn get_install_script() -> &'static str {
    let content = include_str!("scripts/install.sh");
    content
}
