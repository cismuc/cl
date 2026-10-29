use twilight_model::gateway::payload::incoming::GuildCreate;

pub async fn handle(event: Box<GuildCreate>) {
    let (id, name, member_count) = match &*event {
        GuildCreate::Available(guild) => (
            guild.id.to_string(),
            guild.name.as_str(),
            guild.member_count.unwrap_or(0),
        ),
        GuildCreate::Unavailable(guild) => (guild.id.to_string(), "<unavailable>", 0),
    };

    tracing::info!(
        guild_id = %id,
        guild_name = %name,
        members = member_count,
        "Connected to guild"
    );
}
