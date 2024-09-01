mod runners;

use crate::runners::MapRunner;
use axum::{response::Json, routing::get, Router};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, timeout::TimeoutLayer, trace::TraceLayer};
use tracing::trace;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(serde::Serialize)]
struct Answer {
    answer: String,
}

async fn handler() -> Json<Answer> {
    Answer {
        answer: "42".to_string(),
    }
    .into()
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .try_init()
        .expect("could not initialize logging");

    let app = Router::new()
        .route("/", get(handler))
        .map_runner_routes()
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(TimeoutLayer::new(Duration::from_secs(10))),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    trace!(
        "Click'n'Ez Automation v{} - {}",
        VERSION,
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}
