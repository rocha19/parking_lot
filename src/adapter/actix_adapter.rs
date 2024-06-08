use actix_web::{web, Responder};
use std::sync::Arc;

use crate::infra::http::handler::generic_handler::{AdapterRequest, GenericHandler};

#[derive(Clone)]
pub struct ActixHandler {
    pub inner: Arc<dyn GenericHandler + Send + Sync>,
}

impl ActixHandler {
    pub async fn handle(&self, req: web::Json<AdapterRequest>) -> impl Responder {
        let response = self.inner.handle(req.into_inner()).await;
        web::Json(response)
    }
}
