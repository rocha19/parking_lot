use actix_web::middleware::Logger;
use actix_web::{web, App, Error, HttpServer, Responder, Result};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    adapter::actix_adapter::ActixHandler,
    domain::repository::parking_lot_repository::ParkingLotRepository,
    infra::{
        http::handler::{
            generic_handler::AdapterRequest, get_parking_lot_handler::GetParkingLotHandler,
        },
        repository::parking_lot_repository_sqlite::ParkingLotRepositorySqlite,
    },
};

#[actix_web::main]
pub async fn server() -> std::io::Result<()> {
    let parking_lot_repository_sqlite = ParkingLotRepositorySqlite::new("./parking_lot.sqlite");
    parking_lot_repository_sqlite.add_parking_lot();

    let repository: Arc<RwLock<dyn ParkingLotRepository>> =
        Arc::new(RwLock::new(parking_lot_repository_sqlite));

    let handler = Arc::new(GetParkingLotHandler { repository });

    let actix_handler = ActixHandler { inner: handler };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(actix_handler.clone()))
            .service(web::resource("/parking/{code}").route(web::get().to(handle_parking)))
    })
    .keep_alive(actix_web::http::KeepAlive::Os)
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

async fn handle_parking(
    handler: web::Data<ActixHandler>,
    code: web::Path<String>,
) -> Result<impl Responder, Error> {
    let request = AdapterRequest {
        code: code.into_inner(),
    };
    let response = handler.handle(web::Json(request)).await;
    Ok(response)
}
