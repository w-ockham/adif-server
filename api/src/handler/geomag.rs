use axum::{routing::get, Json, Router};
use shaku_axum::Inject;

use crate::model::geomag::GeomagResponse;
use common::error::{AppError, AppResult};
use registry::{AppRegistry, AppState};
use service::services::UserService;

async fn get_geomag(
    user_service: Inject<AppRegistry, dyn UserService>,
) -> AppResult<Json<GeomagResponse>> {
    let result = user_service.get_geomagnetic().await?;
    if let Some(result) = result {
        return Ok(Json(result.into()));
    }
    Err(AppError::EntityNotFound("GeoMag Error".to_string()))
}

pub fn build_geomag_routers() -> Router<AppState> {
    let routers = Router::new().route("/", get(get_geomag));
    Router::new().nest("/geomag", routers)
}
