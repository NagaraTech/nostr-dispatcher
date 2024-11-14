use dispatcher::config::CustomConfig;
use dispatcher::error::handle_error;
use dispatcher::server::server::SharedState;
use axum::error_handling::HandleErrorLayer;
use axum::http::Method;
use axum::routing::get;
use axum::{routing::post, Router};
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;
use tokio::sync::mpsc;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use dispatcher::{router, service};
use tower_http::cors::{Any, CorsLayer};
use tracing::Level;
use tracing_subscriber::EnvFilter;
use dispatcher::models::message;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let custom_config = CustomConfig::from_toml().await;
    let config = dispatcher::config::Config::new().merge(&custom_config.clone());
    let max_level = custom_config.clone()
        .log
        .and_then(|c| c.level)
        .map_or(Level::INFO, |cl| {
            tracing::Level::from_str(&cl).unwrap_or(Level::INFO)
        });
    let server_addr = custom_config.clone()
        .server
        .map_or((String::from("0.0.0.0"), 3000), |s| {
            (s.host.unwrap_or("0.0.0.0".into()), s.port.unwrap_or(3000))
        });
    let addr = format!("{}:{}", server_addr.0, server_addr.1);
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_max_level(max_level)
        .init();
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let (dispatch_messate_tx, dispatch_message_rx) = mpsc::channel::<message::Message>(200);

    let server = SharedState::new(config, dispatch_messate_tx.clone()).await;

    // let nostr_sub_task = tokio::spawn(dispatcher::service::nostr::subscription_service(
    //     server.clone(),
    //     job_status_rx,
    //     dispatch_task_tx.clone(),
    //     secret_key,
    //     custom_config.default_relay.unwrap_or("ws://localhost:8080".into())
    // ));
    let nostr_sub_task = tokio::spawn(dispatcher::service::nostr::sync_message(
        server.clone(),
        dispatch_message_rx,
        custom_config.clone(),
    ));
    // build our application with a single route
    let app = Router::new()
        .route("/api/message/submit", post(router::message::submit))
        .route("/api/message/list", get(router::message::list))
        .with_state(server)
        .layer(cors)
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_error))
                .timeout(Duration::from_secs(600))
                .layer(TraceLayer::new_for_http()),
        );

    let server_task = tokio::spawn(async {
        tracing::info!("start server on {}", addr);
        match tokio::net::TcpListener::bind(addr).await {
            Ok(listener) => {
                match axum::serve(
                    listener,
                    app.into_make_service_with_connect_info::<SocketAddr>(),
                )
                .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        tracing::error!("start server error: {}", e);
                    }
                }
            }
            Err(e) => {
                tracing::error!("start server error: {}", e);
            }
        }
    });

    let (nostr_result, server_result) = tokio::join!(
        nostr_sub_task,
        server_task,
    );
    nostr_result??;
    server_result?;
    Ok(())
}
