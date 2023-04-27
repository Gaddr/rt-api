use crate::{
    auth::auth,
    models::api::{
        CreateDocumentRequest,
    },
    queries::{
        query_add_document, query_get_document_by_name,
    },
    AppState,
};

use actix_web::{
    delete, get, post, put,
    web::{self, Data, Json, Path},
    HttpResponse, Responder, Scope,
};

use sqlx::{self, types::chrono::{DateTime, Utc, NaiveDateTime}};
use uuid::Uuid;

pub fn api_scope() -> Scope {
    return web::scope("")
        .service(create_document);
}

#[post("/new")]
async fn create_document(
    state: Data<AppState>,
    body: Json<CreateDocumentRequest>,
    // _: auth::JwtMiddleware,
) -> impl Responder {
    // Check if document with this name already exists
    let document_exists = query_get_document_by_name(&state, body.name.to_string()).await;

    if document_exists.is_ok() {
        return HttpResponse::BadRequest().json("Doc with this name already exists. Please choose a different name.");
    }

    // Otherwise create new id, and set created_at and modified_at to current timestamp (UTC)
    let id = Uuid::new_v4();
    // TODO: fix: doesnt get current timestamp
    let current_timestamp = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(61, 0).unwrap(), Utc);
    println!("current_timestamp {}", current_timestamp);

    match query_add_document(&state, &id, &body, &current_timestamp).await {
        Ok(_) => HttpResponse::Ok().json("Document created!"),
        Err(err) => {
            if err.to_string().contains("duplicate") {
                return HttpResponse::BadRequest().json("Document already exists!");
            } else {
                if err.to_string().contains("violates foreign key") { // TODO: Please make sure cities etc exist
                    return HttpResponse::BadRequest().json("Error creating document - foreign key violation.");
                } else {
                    return HttpResponse::InternalServerError().json(err.to_string());
                }
            }
        }
    }
}
