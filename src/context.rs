use std::sync::Arc;
use twilight_http::Client as HttpClient;
use twilight_model::id::{marker::UserMarker, Id};

pub struct AppState {
    pub http: Arc<HttpClient>,
    pub bot_id: Id<UserMarker>,
}

pub type Context = Arc<AppState>;
