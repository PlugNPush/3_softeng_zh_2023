use axum::Router;
use clap::Parser;
use models::{Id, TemperatureMeasurement};
use server::{config::Config, docs::docs_handler, frontend::frontend_handler, router::api_router};
use std::net::SocketAddr;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let m = TemperatureMeasurement {
        id: Id::random(),
        timestamp: chrono::DateTime::<chrono::Utc>::default(),
        temperature: 1234,
    };
    println!("{:?}", serde_json::to_string(&m));

    let config = Config::parse();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from(&config.log_level))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let router = Router::new()
        .nest("/api", api_router().layer(TraceLayer::new_for_http()))
        .nest("/docs", Router::new().fallback(docs_handler))
        .fallback(frontend_handler)
        .layer(CompressionLayer::new());

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    tracing::info!("listening on http://{addr}");
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
