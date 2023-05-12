use crate::{
    auth::auth,
    models::api::{ModifyDocumentMetadataRequest, UpdateDocumentRequest},
    queries::{
        query_add_document, query_change_document_metadata, query_delete_document,
        query_get_all_document_names, query_get_document_by_id, query_get_document_by_name,
        query_update_document,
    },
    AppState,
};

use actix_web::{
    delete, get, post, put,
    web::{self, Data, Json, Path},
    HttpResponse, Responder, Scope,
};

use rand::Rng;
use sqlx::{
    self,
    types::chrono::{DateTime, NaiveDateTime, Utc},
};
use serde_json::{Result, Value, json};
use uuid::Uuid;

pub fn api_scope() -> Scope {
    return web::scope("/document")
        .service(create_document)
        .service(get_all_names)
        .service(get_document)
        .service(modify_document_metadata)
        .service(update_document)
        .service(delete_document);
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
    let empty_editor_content = json!({"root":{"children":[{"children":[],"direction":null,"format":"","indent":0,"type":"paragraph","version":1}],"direction":null,"format":"","indent":0,"type":"root","version":1}});

    match query_add_document(&state, &id, &name, &empty_editor_content, &current_timestamp).await {
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
    match query_get_all_document_names(&state).await {
        Ok(document) => return HttpResponse::Ok().json(document),
        Err(err) => HttpResponse::NotFound().json(err.to_string()),
    }
}

#[get("/getById/{id}")]
async fn get_document(state: Data<AppState>, path: Path<String>) -> impl Responder {
    let id: Uuid = match uuid::Uuid::try_parse(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json("Could not parse category id as a UUID!"),
    };

    match query_get_document_by_id(&state, &id).await {
        Ok(document) => return HttpResponse::Ok().json(document),
        Err(err) => return HttpResponse::NotFound().json(err.to_string()),
    };
}

#[post("/modifyMetadata")]
async fn modify_document_metadata(
    state: Data<AppState>,
    body: Json<ModifyDocumentMetadataRequest>,
    // _: auth::JwtMiddleware,
) -> impl Responder {
    // Check if document with this name already exists
    let document_exists = query_get_document_by_name(&state, &body.name.to_string()).await;

    if document_exists.is_ok() {
        return HttpResponse::BadRequest()
            .json("Doc with this name already exists. Please choose a different name.");
    }

    match query_change_document_metadata(&state, &body.id, &body.name).await {
        Ok(document) => return HttpResponse::Ok().json(document),
        Err(_) => HttpResponse::NotFound().json("No courses found"),
    }
}

#[post("/update")]
async fn update_document(
    state: Data<AppState>,
    body: Json<UpdateDocumentRequest>,
    // _: auth::JwtMiddleware,
) -> impl Responder {
    let current_timestamp = Utc::now();
    match query_update_document(&state, &body.id, &body.content, &current_timestamp).await {
        Ok(document) => HttpResponse::Ok().json(document),
        Err(err) => {
            return HttpResponse::InternalServerError().json(err.to_string());
        }
    }
}

#[delete("/delete/{id}")]
async fn delete_document(
    state: Data<AppState>,
    path: Path<String>,
    // _: auth::JwtMiddleware,
) -> impl Responder {
    let id: Uuid = match uuid::Uuid::try_parse(&path.into_inner()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().json("Could not parse category id as a UUID!"),
    };

    match query_delete_document(&state, &id).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => {
            return HttpResponse::InternalServerError().json(err.to_string());
        }
    }
}
