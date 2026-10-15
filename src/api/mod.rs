use std::time::{SystemTime, UNIX_EPOCH};

use poem_openapi::{OpenApi, payload::PlainText};

pub struct Api;

#[OpenApi]
impl Api {
    /// Liveness probe. Always returns `200 OK`.
    #[oai(path = "/health", method = "get")]
    async fn health(&self) -> PlainText<&'static str> {
        PlainText("ok")
    }

    /// Current time as milliseconds since the Unix epoch.
    ///
    /// Compatible with JavaScript's `Date.now()`.
    #[oai(path = "/time", method = "get")]
    async fn time(&self) -> PlainText<String> {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System clock is set before the Unix epoch")
            .as_millis();
        PlainText(millis.to_string())
    }
}
