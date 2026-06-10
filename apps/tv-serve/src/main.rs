use axum::{
    routing::{get, post},
    Json, Router,
};
use tv_engine::Engine;
use tv_kernel::clock::SystemClock;
use tv_kernel::ids::MerchantId;
use tv_kernel::money::{Currency, Money};
use rust_decimal::Decimal;
use std::net::SocketAddr;
use std::sync::Mutex;
use tracing_subscriber::EnvFilter;

#[derive(Debug)]
struct Args {
    listen: SocketAddr,
}

struct AppState {
    engine: Mutex<Engine<SystemClock>>,
}

struct PayRequest {
    amount: Decimal,
    currency: String,
    reference: String,
}

struct PayResponse {
    result: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();
    let args = Args::parse();
    let state = std::sync::Arc::new(AppState {
        engine: Mutex::new(Engine::new(SystemClock)),
    });
    let app = Router::new()
        .route(
            "/health",
            get(|| async { Json(serde_json::json!({"ok": true})) }),
        )
        .route("/v1/payments", post(create_payment))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind(args.listen).await?;
    tracing::info!("listening on {}", args.listen);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn create_payment(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<AppState>>,
    Json(req): Json<PayRequest>,
) -> Result<Json<PayResponse>, (axum::http::StatusCode, String)> {
    let ccy: Currency = req
        .currency
        .parse()
        .map_err(|e: tv_kernel::error::KernelError| {
            (axum::http::StatusCode::BAD_REQUEST, e.to_string())
        })?;
    let amount = Money::new(req.amount, ccy)
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;
    let mut eng = state
        .engine
        .lock()
        .map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "lock".into()))?;
    let result = eng
        .onboard_merchant_payment(MerchantId::new(), amount, &req.reference)
        .map_err(|e| (axum::http::StatusCode::CONFLICT, e.to_string()))?;
    Ok(Json(PayResponse { result }))
}
