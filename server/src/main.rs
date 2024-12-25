use axum::{extract::Query, routing::get, Json, Router};
use common::{CliPrompt, GetCliPromptRequestQuery, GetCliPromptResponse};

#[tokio::main]
async fn main() {
    // converts tracing records to stdout logs
    #[cfg(debug_assertions)] // debug mode
    let max_log_level = tracing::Level::TRACE;
    #[cfg(not(debug_assertions))] // release mode
    let max_log_level = tracing::Level::WARN;
    tracing_subscriber::fmt()
        .with_max_level(max_log_level)
        .init();

    let app = Router::new().route("/cli-prompt", get(get_cli_prompt));

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
    let prompt = CliPrompt {
        value: format!("echo \"{}\"", query.q),
    };

    Json(GetCliPromptResponse { prompt })
}
