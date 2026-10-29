use crate::context::Context;
use twilight_model::gateway::payload::incoming::MessageCreate;

pub async fn handle(ctx: &Context, event: Box<MessageCreate>) {
    // Fast path: Ignore messages sent by bots (including our own bot ID)
    if event.author.bot || event.author.id == ctx.bot_id {
        return;
    }

    let content = event.content.trim();

    // Responsive command check without heavy regex or string allocations
    if content.eq_ignore_ascii_case("!ping") {
        let channel_id = event.channel_id;

        if let Err(err) = ctx
            .http
            .create_message(channel_id)
            .content("pong! 🏓")
            .await
        {
            tracing::error!(
                error = %err,
                channel_id = %channel_id,
                "Failed to send ping response"
            );
        } else {
            tracing::debug!(channel_id = %channel_id, "Responded to ping command");
        }
    }
}
