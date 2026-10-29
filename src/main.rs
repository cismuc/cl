mod config;
mod context;
mod gateway;
mod handlers;

use config::Config;
use context::AppState;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use twilight_http::Client as HttpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Structured logging with RUST_LOG fallback to info
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "discord_bot=info,twilight_gateway=info".into()),
        )
        .with(tracing_subscriber::fmt::layer().compact())
        .init();

    tracing::info!("Initializing optimized Discord bot...");

    // 1. Load validated configuration
    let config = Config::from_env()?;

    // 2. Initialize Twilight HTTP client
    let http = Arc::new(HttpClient::new(config.discord_token.clone()));

    // 3. Resolve current bot user identity
    let current_user = http.current_user().await?.model().await?;
    tracing::info!(
        bot_user = %current_user.name,
        bot_id = %current_user.id,
        "Retrieved bot identity"
    );

    let ctx = Arc::new(AppState {
        http,
        bot_id: current_user.id,
    });

    // 4. Setup graceful termination channel (SIGINT/SIGTERM)
    let (shutdown_tx, shutdown_rx) = tokio::sync::broadcast::channel::<()>(1);

    let shutdown_signal = {
        let shutdown_tx = shutdown_tx.clone();
        tokio::spawn(async move {
            tokio::signal::ctrl_c().await.ok();
            tracing::info!("Shutdown signal received (Ctrl+C)");
            let _ = shutdown_tx.send(());
        })
    };

    // 5. Run gateway shard loop
    if let Err(e) = gateway::run_shard(config.discord_token, ctx, shutdown_rx).await {
        tracing::error!(error = %e, "Gateway terminated unexpectedly");
    }

    let _ = shutdown_signal.await;
    tracing::info!("Bot shutdown complete");

    Ok(())
}
