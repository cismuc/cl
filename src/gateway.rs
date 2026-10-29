use crate::context::Context;
use crate::handlers::route_event;
use twilight_gateway::error::ReceiveMessageErrorType;
use twilight_gateway::{
    CloseFrame, ConfigBuilder, EventTypeFlags, Intents, Shard, ShardId, StreamExt as _,
};
use twilight_model::gateway::payload::outgoing::update_presence::UpdatePresencePayload;
use twilight_model::gateway::presence::{Activity, ActivityType, Status};

fn build_custom_presence(
    status_text: &str,
) -> Result<UpdatePresencePayload, Box<dyn std::error::Error + Send + Sync>> {
    let custom_activity = Activity {
        application_id: None,
        assets: None,
        buttons: Vec::new(),
        created_at: None,
        details: None,
        emoji: None,
        flags: None,
        id: None,
        instance: None,
        kind: ActivityType::Custom,
        name: "Custom Status".to_string(),
        party: None,
        secrets: None,
        state: Some(status_text.to_string()),
        timestamps: None,
        url: None,
    };

    let presence = UpdatePresencePayload::new(vec![custom_activity], false, None, Status::Online)?;

    Ok(presence)
}

pub async fn run_shard(
    token: String,
    ctx: Context,
    mut shutdown_rx: tokio::sync::broadcast::Receiver<()>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Only subscribe to strictly required intents: GUILDS, GUILD_MESSAGES, MESSAGE_CONTENT
    let intents = Intents::GUILDS | Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT;

    // Filter gateway incoming deserialization to only the events we need (eliminates useless JSON decoding)
    let event_filter = EventTypeFlags::READY
        | EventTypeFlags::GUILD_CREATE
        | EventTypeFlags::MESSAGE_CREATE
        | EventTypeFlags::RESUMED;

    let presence = build_custom_presence("hi")?;

    let config = ConfigBuilder::new(token, intents)
        .presence(presence)
        .build();

    let mut shard = Shard::with_config(ShardId::ONE, config);

    tracing::info!("Starting Discord gateway shard 0 with custom presence...");

    loop {
        tokio::select! {
            _ = shutdown_rx.recv() => {
                tracing::info!("Received shutdown signal. Sending normal gateway close frame...");
                shard.close(CloseFrame::NORMAL);
                break;
            }
            event_opt = shard.next_event(event_filter) => {
                let Some(event_res) = event_opt else {
                    tracing::info!("Shard stream closed");
                    break;
                };

                match event_res {
                    Ok(event) => {
                        let ctx_clone = ctx.clone();
                        // Dispatch to task worker to keep the WebSocket stream unblocked
                        tokio::spawn(async move {
                            route_event(&ctx_clone, event).await;
                        });
                    }
                    Err(source) => {
                        tracing::warn!(error = %source, "Gateway event error occurred");
                        if matches!(source.kind(), ReceiveMessageErrorType::Reconnect) {
                            tracing::error!("Fatal gateway reconnect failure, terminating shard");
                            return Err(source.into());
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
