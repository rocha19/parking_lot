use axum::Json;
use std::sync::Arc;

use crate::infra::http::handler::generic_handler::{
    AdapterRequest, AdapterResponse, GenericHandler,
};

pub struct AxumHandler {
    pub inner: Arc<dyn GenericHandler + Send + Sync>,
}

pub async fn handle(
    axum_handler: Arc<AxumHandler>,
    Json(req): Json<AdapterRequest>,
) -> Json<AdapterResponse> {
    let response = axum_handler.inner.handle(req).await;
    Json(response)
}
