use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use axum::{body::Body, http::Request, middleware::Next, response::Response, Json};
use tracing::info;
use uuid::Uuid;
use axum::{Router, routing::get, middleware::from_fn};
use serde_json::json;
use tokio::signal;


 // Seting up tracing with JSON output format
 fn init_tracing() {
        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::from_default_env())
            .with(tracing_subscriber::fmt::layer().json())
            .init();
 }

// Creating middleware to inject a request ID into each request
 async fn request_id_middleware(req: Request<Body>, next: Next) -> Response {
     let request_id = Uuid::new_v4().to_string();
     info!(request_id, "Handling request");
     next.run(req).await



 }

// Creating a simple health check endpoint

 async fn healthz() -> Json<serde_json::Value> {
     Json(json!({ "ok": true }))
 }

 fn app() -> Router {
        Router::new()
            .route("/healthz", get(healthz))
            .layer(from_fn(request_id_middleware))
 }

//  Graceful shutdown signal handler
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


}
 


#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    init_tracing();

    let app = app();
    let addr = std::env::var("PORT").unwrap_or_else(|_| "3001".into());

    info!("🚀 Starting server on 0.0.0.0:{}", addr);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", addr))
        .await
        .expect("❌ Server failed to start");


    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("❌ Failed to bind to address");

}

    

