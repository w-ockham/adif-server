use axum::{
    extract::{Multipart, Path, Query},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};

use chrono::{Duration, TimeZone, Utc};
use shaku_axum::Inject;
use std::str::FromStr;

use common::error::{AppError, AppResult};

use domain::model::sota::SummitCode;
use domain::model::{
    event::{DeleteRef, FindActBuilder, FindLogBuilder, FindRefBuilder},
    id::UserId,
};

use registry::{AppRegistry, AppState};
use service::model::sota::{UploadSOTALog, UploadSOTASummit, UploadSOTASummitOpt};
use service::services::{AdminService, UserService};

use crate::model::{
    activation::ActivationResponse,
    alerts::AlertResponse,
    param::{build_findref_query, GetParam},
    spots::SpotResponse,
};

use crate::model::sota::{PagenatedResponse, SOTARefResponse, UpdateRefRequest};

async fn update_sota_reference(
    admin_service: Inject<AppRegistry, dyn AdminService>,
    Json(req): Json<UpdateRefRequest>,
) -> AppResult<StatusCode> {
    admin_service
        .update_sota_reference(req.into())
        .await
        .map(|_| StatusCode::CREATED)
}

async fn import_summit_list(
    admin_service: Inject<AppRegistry, dyn AdminService>,
    mut multipart: Multipart,
) -> AppResult<StatusCode> {
    if let Some(field) = multipart.next_field().await.unwrap() {
        let data = field.bytes().await.unwrap();
        let data = String::from_utf8(data.to_vec()).unwrap();

        let reqs = UploadSOTASummit { data };

        return admin_service
            .import_summit_list(reqs)
            .await
            .map(|_| StatusCode::CREATED);
    }
    Err(AppError::ForbiddenOperation)
}

async fn update_summit_list(
    admin_service: Inject<AppRegistry, dyn AdminService>,
    mut multipart: Multipart,
) -> AppResult<StatusCode> {
    if let Some(field) = multipart.next_field().await.unwrap() {
        let data = field.bytes().await.unwrap();
        let data = String::from_utf8(data.to_vec()).unwrap();

        let reqs = UploadSOTASummit { data };

        return admin_service
            .update_summit_list(reqs)
            .await
            .map(|_| StatusCode::CREATED);
    }
    Err(AppError::ForbiddenOperation)
}

async fn import_sota_opt_reference(
    admin_service: Inject<AppRegistry, dyn AdminService>,
    mut multipart: Multipart,
) -> AppResult<StatusCode> {
    if let Some(field) = multipart.next_field().await.unwrap() {
        let data = field.bytes().await.unwrap();
        let data = String::from_utf8(data.to_vec()).unwrap();

        let reqs = UploadSOTASummitOpt { data };

        return admin_service
            .import_summit_opt_list(reqs)
            .await
            .map(|_| StatusCode::CREATED);
    }
    Err(AppError::ForbiddenOperation)
}

async fn upload_log(
    user_service: Inject<AppRegistry, dyn UserService>,
    Path(user_id): Path<String>,
    mut multipart: Multipart,
) -> AppResult<StatusCode> {
    if let Some(field) = multipart.next_field().await.unwrap() {
        let data = field.bytes().await.unwrap();
        let data = String::from_utf8(data.to_vec()).unwrap();
        let user_id = UserId::from_str(&user_id)?;
        let reqs = UploadSOTALog { data };

        return user_service
            .upload_sota_csv(user_id, reqs)
            .await
            .map(|_| StatusCode::CREATED);
    }
    Err(AppError::ForbiddenOperation)
}

async fn delete_log(
    user_service: Inject<AppRegistry, dyn UserService>,
    Path(user_id): Path<String>,
) -> AppResult<StatusCode> {
    let user_id = UserId::from_str(&user_id)?;
    user_service
        .delete_sota_log(user_id)
        .await
        .map(|_| StatusCode::OK)
}

async fn show_progress(
    user_service: Inject<AppRegistry, dyn UserService>,
    Path(user_id): Path<String>,
) -> AppResult<Json<String>> {
    let user_id = UserId::from_str(&user_id)?;
    let mut query = FindLogBuilder::default();
    let from = Utc.with_ymd_and_hms(2024, 7, 1, 0, 0, 0).unwrap();
    let to = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();

    query = query.after(from).before(to);
    let query = query.build();

    let result = user_service.award_progress(user_id, query).await?;
    Ok(Json(result))
}

async fn delete_sota_reference(
    admin_service: Inject<AppRegistry, dyn AdminService>,
    Path(summit_code): Path<String>,
) -> AppResult<StatusCode> {
    let req = DeleteRef::Delete(SummitCode::new(summit_code));
    admin_service
        .delete_sota_reference(req)
        .await
        .map(|_| StatusCode::OK)
}

async fn show_sota_reference(
    admin_service: Inject<AppRegistry, dyn AdminService>,
    Path(summit_code): Path<String>,
) -> AppResult<Json<SOTARefResponse>> {
    let query = FindRefBuilder::default()
        .sota()
        .sota_code(summit_code)
        .build();
    let result = admin_service.show_sota_reference(query).await?;
    Ok(Json(result.into()))
}

async fn show_all_sota_reference(
    admin_service: Inject<AppRegistry, dyn AdminService>,
    Query(param): Query<GetParam>,
) -> AppResult<Json<PagenatedResponse<SOTARefResponse>>> {
    let mut query = FindRefBuilder::default().sota();
    if param.limit.is_some() {
        query = query.limit(param.limit.unwrap());
    }

    if param.offset.is_some() {
        query = query.offset(param.offset.unwrap());
    }
    let result = admin_service
        .show_all_sota_references(query.build())
        .await?;
    Ok(Json(result.into()))
}

async fn search_sota_reference(
    user_service: Inject<AppRegistry, dyn UserService>,
    Query(param): Query<GetParam>,
) -> AppResult<Json<Vec<SOTARefResponse>>> {
    let query = FindRefBuilder::default().sota();
    let query = build_findref_query(param, query)?;

    let results = user_service.find_references(query).await?;

    let res: Vec<_> = results
        .sota
        .unwrap_or(vec![])
        .into_iter()
        .map(SOTARefResponse::from)
        .collect();
    Ok(Json(res))
}

async fn show_sota_spots(
    user_service: Inject<AppRegistry, dyn UserService>,
    Query(param): Query<GetParam>,
) -> AppResult<Json<Vec<ActivationResponse<SpotResponse>>>> {
    let hours = param.hours_ago.unwrap_or(3);
    let query = FindActBuilder::default()
        .sota()
        .issued_after(Utc::now() - Duration::hours(hours))
        .build();
    let result = user_service.find_spots(query).await?;
    let spots: Vec<_> = result
        .into_iter()
        .map(|(k, v)| {
            ActivationResponse::from((k, v.into_iter().map(SpotResponse::from).collect::<Vec<_>>()))
        })
        .collect();
    Ok(Json(spots))
}

async fn show_sota_alerts(
    user_service: Inject<AppRegistry, dyn UserService>,
    Query(param): Query<GetParam>,
) -> AppResult<Json<Vec<ActivationResponse<AlertResponse>>>> {
    let hours = param.hours_ago.unwrap_or(3);
    let query = FindActBuilder::default()
        .sota()
        .issued_after(Utc::now() - Duration::hours(hours))
        .build();
    let result = user_service.find_alerts(query).await?;
    let alerts: Vec<_> = result
        .into_iter()
        .map(|(k, v)| {
            ActivationResponse::from((
                k,
                v.into_iter().map(AlertResponse::from).collect::<Vec<_>>(),
            ))
        })
        .collect();
    Ok(Json(alerts))
}

pub fn build_sota_routers() -> Router<AppState> {
    let routers = Router::new()
        .route("/import", post(import_summit_list))
        .route("/import/ja", post(import_sota_opt_reference))
        .route("/log/{user_id}", post(upload_log))
        .route("/log/{user_id}", delete(delete_log))
        .route("/log/{user_id}", get(show_progress))
        .route("/update", post(update_summit_list))
        .route("/spots", get(show_sota_spots))
        .route("/alerts", get(show_sota_alerts))
        .route("/summits", get(show_all_sota_reference))
        .route("/summits/search", get(search_sota_reference))
        .route("/summits/{summit_code}", get(show_sota_reference))
        .route("/summits/{summit_code}", put(update_sota_reference))
        .route("/summits/{summit_code}", delete(delete_sota_reference));

    Router::new().nest("/sota", routers)
}
