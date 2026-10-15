use poem_openapi::{OpenApi, payload::PlainText};

pub struct Api;

#[OpenApi]
impl Api {
    /// Liveness probe. Always returns `200 OK`.
    #[oai(path = "/health", method = "get")]
    async fn health(&self) -> PlainText<&'static str> {
        PlainText("ok")
    }
}
