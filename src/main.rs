use anyhow::Context;
use axum::{
    Json, Router, extract,
    http::{StatusCode, header::HeaderMap},
    routing::post,
};
use serde::Deserialize;
use std::{net::SocketAddr, sync::OnceLock};
use timetagger_shortcuts::*;
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct CreateRecord {
    ds: String,
    filter: Option<String>,
}

static BASE_URL: OnceLock<String> = OnceLock::new();

async fn create(
    headers: HeaderMap,
    extract::Json(payload): extract::Json<CreateRecord>,
) -> Result<Json<RecordPutResp>, StatusCode> {
    let authtoken = headers.get("authtoken").ok_or(StatusCode::FORBIDDEN)?;
    let api_client = APIClient::new(
        BASE_URL
            .get()
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
            .clone(),
        authtoken
            .to_str()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .to_string(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let running_records = api_client
        .get_running_records()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if let Some(filter) = payload.filter {
        // Need to check whether there's a running record with matching tags

        let tags = Record::str_to_tags(&filter);
        for r in &running_records {
            if r.tags().is_superset(&tags) {
                let resp = api_client
                    .put_records(vec![])
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                return Ok(Json(resp));
            }
        }
    }
    // Either we don't need to check the running records, or we have finished checking, and there's none that matches
    let stop_running_records: Vec<Record> = running_records
        .into_iter()
        .filter(|r| r.is_running())
        .map(|r| r.stop())
        .collect();
    api_client
        .put_records(stop_running_records)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let new_records = vec![Record::new(payload.ds)];
    let resp = api_client
        .put_records(new_records)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(resp))
}

#[derive(Deserialize)]
struct StopRecord {
    filter: Option<String>,
}

async fn stop(
    headers: HeaderMap,
    extract::Json(payload): extract::Json<StopRecord>,
) -> Result<Json<RecordPutResp>, StatusCode> {
    let authtoken = headers.get("authtoken").ok_or(StatusCode::FORBIDDEN)?;
    let api_client = APIClient::new(
        BASE_URL
            .get()
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
            .clone(),
        authtoken
            .to_str()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .to_string(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let running_records = api_client
        .get_running_records()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let stop_running_records = running_records
        .into_iter()
        .filter(|r| r.is_running())
        .filter(|r| {
            if let Some(sr) = &payload.filter {
                let tags = Record::str_to_tags(sr);
                r.tags().is_superset(&tags)
            } else {
                true
            }
        })
        .map(|r| r.stop())
        .collect();
    let resp = api_client
        .put_records(stop_running_records)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(resp))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env_file = EnvFile::from(".env");
    BASE_URL
        .set(
            env_file
                .base_url
                .context(".env file doesn't have BASE_URL set")?,
        )
        .expect("Fail to set BASE_URL");
    let app = Router::new()
        .route("/create", post(create))
        .route("/stop", post(stop));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8003));
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
