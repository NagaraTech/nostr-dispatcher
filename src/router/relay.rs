use crate::channel::RelayCommand;
use crate::error::AppError;
use crate::models::message::{CreateMessage, Message, MessageFilter};
use crate::models::record::{Record, RecordFilter};
use crate::models::relays::{CreateRelays, Relays, RelaysFilter};
use crate::server::server::SharedState;
use axum::extract::{Query, State};
use axum::{debug_handler, Json};
use serde_json::{json, Value};


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RelayRegister {
    pub url: String,
    pub info: Value,
}


pub async fn register(
    State(server): State<SharedState>,
    Json(req): Json<Value>,
) -> anyhow::Result<Json<Value>, AppError> {
    tracing::debug!("submit job");
    let server = server.0.write().await;
    let relays_tx = server.relays_tx.clone();
    let uuid = uuid::Uuid::new_v4();
    let id = uuid.to_string();
    let ms: RelayRegister = serde_json::from_value(req.clone())?;
    let cm = CreateRelays {
        id: id,
        info: ms.info,
        created_at: chrono::Local::now().naive_utc(),
        url: ms.url,  
    };
    let mut conn = match server.pg.get() {
        Ok(conn) => conn,
        Err(e) => {
            tracing::error!("Failed to get a database connection: {:?}", e);
            return Ok(Json(json!({
                "code": 500,
                "message": "",
            })));
        }
    };

    let m = Relays::create(&mut conn, &cm)?;
    // dispatch task

    if let Err(err) = relays_tx.send(RelayCommand::Add(m.clone())).await {
        tracing::error!("add relay {}", err);
    }

    Ok(Json(json!({
        "code": 200,
        "result": m,
    })))
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RelayRemove {
    pub id: String,
}


pub async fn remove(
    State(server): State<SharedState>,
    Json(req): Json<Value>,
) -> anyhow::Result<Json<Value>, AppError> {
    tracing::debug!("submit job");
    let server = server.0.write().await;
    let relays_tx = server.relays_tx.clone();
    let ms: RelayRemove = serde_json::from_value(req.clone())?;
    let mut conn = match server.pg.get() {
        Ok(conn) => conn,
        Err(e) => {
            tracing::error!("Failed to get a database connection: {:?}", e);
            return Ok(Json(json!({
                "code": 500,
                "message": "",
            })));
        }
    };
    let relays = Relays::read(&mut conn, ms.id)?;

    Relays::delete(&mut conn, relays.id.clone())?;
    // dispatch task

    if let Err(err) = relays_tx.send(RelayCommand::Remove(relays.clone())).await {
        tracing::error!("remove relay {}", err);
    }

    Ok(Json(json!({
        "code": 200,
        "result": relays,
    })))
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PageParams {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

pub async fn list(
    State(server): State<SharedState>,
    Query(page_params): Query<PageParams>,
) -> anyhow::Result<Json<Value>, AppError> {
    let server = server.0.write().await;
    let mut conn = server.pg.get()?;
    let filter = RelaysFilter {
        ..Default::default()
    };
    let page = page_params.page.unwrap_or(1);
    let page_size = page_params.page_size.unwrap_or(10);

    let mut r = Relays::paginate(&mut conn, page - 1, page_size, filter)?;
    r.page = r.page + 1;
    Ok(Json(json!({
        "result": r,
    })))
}



