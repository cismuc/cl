pub mod guild;
pub mod message;

use crate::context::Context;
use twilight_gateway::Event;

pub async fn route_event(ctx: &Context, event: Event) {
    match event {
        Event::Ready(ready) => {
            tracing::info!(
                user = %ready.user.name,
                user_id = %ready.user.id,
                guilds = ready.guilds.len(),
                "Shard successfully authenticated with Discord Gateway"
            );
        }
        Event::GuildCreate(guild) => {
            guild::handle(guild).await;
        }
        Event::MessageCreate(msg) => {
            message::handle(ctx, msg).await;
        }
        Event::Resumed => {
            tracing::info!("Gateway connection successfully resumed");
        }
        // Discard unhandled events without retention or allocation
        _ => {}
    }
}
