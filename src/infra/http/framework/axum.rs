use axum::{extract::Path, routing::get, Json, Router};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::trace::TraceLayer;
use tracing_subscriber;

use crate::{
    adapter::axum_adapter::{handle, AxumHandler},
    domain::repository::parking_lot_repository::ParkingLotRepository,
    infra::{
        http::handler::{
            generic_handler::{AdapterRequest, AdapterResponse},
            get_parking_lot_handler::GetParkingLotHandler,
        },
        repository::parking_lot_repository_sqlite::ParkingLotRepositorySqlite,
    },
};

#[tokio::main]
pub async fn server() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let parking_lot_repository_sqlite = ParkingLotRepositorySqlite::new("./parking_lot.sqlite");
    parking_lot_repository_sqlite.add_parking_lot();

    let repository: Arc<RwLock<dyn ParkingLotRepository>> =
        Arc::new(RwLock::new(parking_lot_repository_sqlite));

    let handler = Arc::new(GetParkingLotHandler { repository });

    let axum_handler = Arc::new(AxumHandler { inner: handler });

    let app = Router::new()
        .route(
            "/parking/:code",
            get(move |code: Path<String>| handle_parking(axum_handler.clone(), code)),
        )
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn handle_parking(
    handler: Arc<AxumHandler>,
    Path(code): Path<String>,
) -> Json<AdapterResponse> {
    let request = AdapterRequest { code };
    handle(handler, Json(request)).await
}
