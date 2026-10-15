#![forbid(unsafe_code)]

use anyhow::{Context, Result};

use dotenvy::dotenv;
use poem::{EndpointExt, Route, Server, listener::TcpListener, middleware::Cors};
use poem_openapi::OpenApiService;

use time_server::api;

fn parse_env_var_array(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn main() -> Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("Failed to build tokio runtime")?
        .block_on(async_main())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

async fn async_main() -> Result<()> {
    dotenv().ok();

    let bind_addr = std::env::var("BIND_ADDR").unwrap_or("0.0.0.0:3000".to_string());
    let server_addr = std::env::var("SERVER_ADDR").unwrap_or("http://localhost:3000/".to_string());
    let cors_allowed_origins = std::env::var("CORS_ALLOWED_ORIGINS").unwrap_or("*".to_string());

    let api_service =
        OpenApiService::new(api::Api, "time-server", env!("CARGO_PKG_VERSION")).server(server_addr);
    let ui = api_service.swagger_ui();

    let spec = api_service.spec_endpoint();

    let cors = if cors_allowed_origins == "*" {
        Cors::new()
    } else {
        Cors::new().allow_origins(parse_env_var_array(&cors_allowed_origins))
    };

    let route = Route::new()
        .nest("/api", api_service)
        .nest("/swagger", ui)
        .at("/openapi.json", spec)
        .with(cors);

    Server::new(TcpListener::bind(bind_addr))
        .run_with_graceful_shutdown(
            route,
            shutdown_signal(),
            Some(std::time::Duration::from_secs(10)),
        )
        .await?;

    Ok(())
}
