use crate::{
    auth::auth,
    models::api::{CreateDocumentRequest, GetDocumentsResponse},
    queries::{query_add_document, query_get_all_documents, query_get_document_by_name},
    AppState,
};

use actix_web::{
    delete, get, post, put,
    web::{self, Data, Json, Path},
    HttpResponse, Responder, Scope,
};

use sqlx::{
    self,
    types::chrono::{DateTime, NaiveDateTime, Utc},
};
use uuid::Uuid;
use rand::Rng;

pub fn api_scope() -> Scope {
    return web::scope("")
        .service(create_document)
        .service(get_all_names);
}

#[get("/create")]
async fn create_document(
    state: Data<AppState>,
    // _: auth::JwtMiddleware,
) -> impl Responder {
    let mut rng = rand::thread_rng();

    // create new id and name, and set created_at and modified_at to current timestamp (UTC)
    let id = Uuid::new_v4();
    let random_number = rng.gen_range(0..100).to_string();
    let name = "New_".to_string() + &random_number;
    let current_timestamp = Utc::now();

    match query_add_document(&state, &id, &name, &current_timestamp).await {
        Ok(_) => HttpResponse::Ok().json("Document created!"),
        Err(err) => {
            if err.to_string().contains("duplicate") {
                return HttpResponse::BadRequest().json("Document already exists!");
            } else {
                return HttpResponse::InternalServerError().json(err.to_string());
            }
        }
    }
}

#[get("/getAllNames")]
async fn get_all_names(state: Data<AppState>) -> impl Responder {
    match query_get_all_documents(&state).await {
        Ok(documents) => {
            let shortened = documents
                .into_iter()
                .map(|doc| GetDocumentsResponse {
                    id: doc.id,
                    name: doc.name,
                    modified_at: doc.modified_at,
                })
                .collect::<Vec<GetDocumentsResponse>>();

            return HttpResponse::Ok().json(shortened);
        }
        Err(_) => HttpResponse::NotFound().json("No courses found"),
    }
}

#[post("/modifyDocumentMetadata")]
async fn modify_document_metadata(
    state: Data<AppState>,
    body: Json<CreateDocumentRequest>,
    // _: auth::JwtMiddleware,
) -> impl Responder {
    // Check if document with this name already exists
    let document_exists = query_get_document_by_name(&state, body.name.to_string()).await;

    if document_exists.is_ok() {
        return HttpResponse::BadRequest()
            .json("Doc with this name already exists. Please choose a different name.");
    }

    // TODO: otherwise change the name. the below is wrong 
    match query_get_all_documents(&state).await {
        Ok(documents) => {
            let shortened = documents
                .into_iter()
                .map(|doc| GetDocumentsResponse {
                    id: doc.id,
                    name: doc.name,
                    modified_at: doc.modified_at,
                })
                .collect::<Vec<GetDocumentsResponse>>();

            return HttpResponse::Ok().json(shortened);
        }
        Err(_) => HttpResponse::NotFound().json("No courses found"),
    }
}
