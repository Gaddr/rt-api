use crate::{
    models::{api::{ModifyDocumentMetadataRequest}, db::{Document, GetDocumentNamesResponse}},
    AppState,
};
use ::chrono::{DateTime, FixedOffset};
use actix_web::web::{Data, Json};
use serde_json::{json, Value};
use sqlx::{self, types::chrono::Utc};
use uuid::Uuid;

pub async fn query_add_document(
    state: &Data<AppState>,
    id: &Uuid,
    name: &String,
    current_timestamp: &DateTime<Utc>,
) -> Result<(), sqlx::Error> {
    let mut tx = state.db.begin().await?;

    let result_create_document = sqlx::query(
        "INSERT INTO grt.document (id, name, created_at, modified_at) VALUES ($1, $2, $3, $4)",
    )
    .bind(&id)
    .bind(&name)
    .bind(&current_timestamp)
    .bind(&current_timestamp)
    .execute(&mut tx)
    .await?
    .rows_affected();

    if result_create_document == 0 {
        return tx.rollback().await;
    }

    let result = tx.commit().await;

    return result;
}

pub async fn query_get_all_document_names(state: &Data<AppState>) -> Result<Vec<GetDocumentNamesResponse>, sqlx::Error> {
    let result = sqlx::query_as::<_, GetDocumentNamesResponse>("SELECT * FROM grt.document")
        .fetch_all(&state.db)
        .await;
    return result;
}

// TODO: keep in json
pub async fn query_get_document_by_id(
    state: &Data<AppState>,
    id: &Uuid,
) -> Result<Document, sqlx::Error> {
    let result = sqlx::query_as::<_, Document>("SELECT * FROM grt.document WHERE id=$1")
        .bind(&id)
        .fetch_one(&state.db)
        .await;

    return result;
}

pub async fn query_get_document_by_name(
    state: &Data<AppState>,
    name: &String,
) -> Result<Document, sqlx::Error> {
    let result = sqlx::query_as::<_, Document>("SELECT * FROM grt.document WHERE name = $1")
        .bind(name)
        .fetch_one(&state.db)
        .await;

    return result;
}

pub async fn query_update_document(
    state: &Data<AppState>,
    id: &Uuid,
    content: &Value,
    current_timestamp: &DateTime<Utc>,
) -> Result<(), sqlx::Error> {
    let mut tx = state.db.begin().await?;

    let result_update_document =
        sqlx::query("UPDATE grt.document SET content=($2::jsonb), modified_at=$3 WHERE id=$1")
            .bind(&id)
            .bind(&content)
            .bind(&current_timestamp)
            .execute(&mut tx)
            .await?
            .rows_affected();

    if result_update_document == 0 {
        return tx.rollback().await;
    }

    let result = tx.commit().await;

    return result;
}
