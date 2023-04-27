use crate::{
    models::{
        api::{
            CreateDocumentRequest,
        },
        db::{
            Document,
        },
    },
    AppState,
};
use ::chrono::{DateTime, FixedOffset};
use actix_web::web::{Data, Json};
use sqlx::{self, types::chrono::Utc};
use uuid::Uuid;

pub async fn query_add_document(
    state: &Data<AppState>,
    id: &Uuid,
    document_details: &Json<CreateDocumentRequest>,
    current_timestamp: &DateTime<Utc>,
) -> Result<(), sqlx::Error> {
    let mut tx = state.db.begin().await?;

    let result_create_document = sqlx::query("INSERT INTO grt.document (id, name, created_at, modified_at) VALUES ($1, $2, $3, $4)")
            .bind(&id)
            .bind(&document_details.name.to_string())
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

pub async fn query_get_document_by_name(
    state: &Data<AppState>,
    name: String,
) -> Result<Document, sqlx::Error> {
    let result = sqlx::query_as::<_, Document>("SELECT * FROM grt.document WHERE name = $1")
        .bind(name)
        .fetch_one(&state.db)
        .await;

    return result;
}
