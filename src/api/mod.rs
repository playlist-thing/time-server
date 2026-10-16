use std::time::{SystemTime, UNIX_EPOCH};

use poem_openapi::{ApiResponse, OpenApi, payload::PlainText};

pub struct Api;

#[derive(ApiResponse)]
enum TimeResponse {
    #[oai(status = 200)]
    Time(PlainText<String>, #[oai(header = "Cache-Control")] String),
}

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
    async fn time(&self) -> TimeResponse {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System clock is set before the Unix epoch")
            .as_millis();

        TimeResponse::Time(PlainText(millis.to_string()), "no-store".to_string())
    }
}
