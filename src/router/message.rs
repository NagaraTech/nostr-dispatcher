use crate::error::AppError;
use crate::models::message::{CreateMessage, Message, MessageFilter};
use crate::models::record::{Record, RecordFilter};
use crate::server::server::SharedState;
use axum::extract::{Query, State};
use axum::{debug_handler, Json};
use serde_json::{json, Value};


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MessageSubmit {
    pub from: String,
    /// Field representing column `to`
    pub to: String,
    /// Field representing column `action`
    pub action: String,
}


pub async fn submit(
    State(server): State<SharedState>,
    Json(req): Json<Value>,
) -> anyhow::Result<Json<Value>, AppError> {
    tracing::debug!("submit job");
    let server = server.0.write().await;
    let dispatch_tx = server.dispatch_task_tx.clone().unwrap();
    let uuid = uuid::Uuid::new_v4();
    let id = uuid.to_string();
    let ms: MessageSubmit = serde_json::from_value(req.clone())?;
    let cm = CreateMessage {
        id: id,
        from: ms.from,
        to: ms.to,
        action: ms.action,
        status: "pending".to_string(),
        info: req,
        created_at: chrono::Local::now().naive_utc(),  
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

    let m = Message::create(&mut conn, &cm)?;
    // dispatch task

    if let Err(err) = dispatch_tx.send(m.clone()).await {
        tracing::error!("dispatch task when submit job {}", err);
    }

    Ok(Json(json!({
        "code": 200,
        "result": m,
    })))
}



#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MessageListParams {
    pub id: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub action: Option<String>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,

}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PageParams {
    pub page: Option<i64>,
    pub page_size: Option<i64>,

}

pub async fn list(
    State(server): State<SharedState>,
    Query(list_params): Query<MessageListParams>,
    Query(page_params): Query<PageParams>,
) -> anyhow::Result<Json<Value>, AppError> {
    let server = server.0.write().await;
    let mut conn = server.pg.get()?;
    let filter = MessageFilter {
        id: list_params.id,
        from: list_params.from,
        to: list_params.to,
        action: list_params.action,
        status: list_params.status,
        ..Default::default()
    };
    let page = page_params.page.unwrap_or(1);
    let page_size = page_params.page_size.unwrap_or(10);

    let mut r = Message::paginate(&mut conn, page - 1, page_size, filter)?;
    r.page = r.page + 1;
    Ok(Json(json!({
        "result": r,
    })))
}



#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RecordListParams {
    pub id: Option<String>,
    pub event_id: Option<String>,
    pub relay: Option<String>,
    pub message_id: Option<String>,
    pub status: Option<String>,
}

pub async fn record(
    State(server): State<SharedState>,
    Query(list_params): Query<RecordListParams>,
    Query(page_params): Query<PageParams>,
) -> anyhow::Result<Json<Value>, AppError> {
    let server = server.0.write().await;
    let mut conn = server.pg.get()?;
    let filter = RecordFilter {
        id: list_params.id,
        message_id: list_params.message_id,
        event_id: list_params.event_id,
        status: list_params.status,
        relay: list_params.relay,
        ..Default::default()
    };
    let page = page_params.page.unwrap_or(1);
    let page_size = page_params.page_size.unwrap_or(10);

    let mut r = Record::paginate(&mut conn, page - 1, page_size, filter)?;
    r.page = r.page + 1;
    Ok(Json(json!({
        "result": r,
    })))
}