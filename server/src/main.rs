use axum::{error_handling::HandleErrorLayer, http, Router};
use clap::Parser;
use server::{config::Config, docs::docs_handler, frontend::frontend_handler, router::api_router};
use std::net::SocketAddr;
use tokio::signal;
use tower::{BoxError, ServiceBuilder};
use tower_governor::{errors::display_error, governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::{compression::CompressionLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let config = Config::parse();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from(&config.log_level))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // rate limiter stuff is mostly copied from the example
    // https://docs.rs/tower_governor/0.1.0/tower_governor/
    let rate_limiter_config = Box::new(
        GovernorConfigBuilder::default()
            .per_second(1)
            .burst_size(2)
            .methods(vec![http::Method::POST])
            .finish()
            .unwrap(),
    );
    let rate_limiter_layer = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e: BoxError| async move {
            display_error(e)
        }))
        .layer(GovernorLayer {
            config: Box::leak(rate_limiter_config),
        });

    let router = Router::new()
        .nest("/api", api_router().layer(TraceLayer::new_for_http()))
        .nest("/docs", Router::new().fallback(docs_handler))
        .fallback(frontend_handler)
        .layer(CompressionLayer::new())
        .layer(rate_limiter_layer);

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("listening on http://{addr}");
    axum::Server::bind(&addr)
        .serve(router.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

/// Handle SIGTERM and SIGINT on our own. This is needed for the process to behave properly when
/// running as PID 1 (as is the case as the sole thing in a container).
///
/// Includes configuration for non-unix systems but that is untested as well as not expected to be
/// used.
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    tracing::info!("signal received, shutting down...");
}
