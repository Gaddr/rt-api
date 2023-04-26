use crate::{
    auth::auth,
    models::api::{
        CreateCourseRequest,
    },
    queries::{
        query_add_course, query_get_course_by_name,
    },
    AppState,
};

use actix_web::{
    delete, get, post, put,
    web::{self, Data, Json, Path},
    HttpResponse, Responder, Scope,
};

use sqlx::{self, types::chrono::DateTime};
use uuid::Uuid;

// TODO: remove this admin scope
pub fn get_admin_scope() -> Scope {
    return web::scope("/admin")
        .service(create_course)
}

#[post("/course")]
async fn create_course(
    state: Data<AppState>,
    body: Json<CreateCourseRequest>,
    _: auth::JwtMiddleware,
) -> impl Responder {
    // Check if course with this name already exists
    let course_exists = query_get_course_by_name(&state, body.course_name.to_string()).await;

    if course_exists.is_ok() {
        return HttpResponse::BadRequest().json("Course with this name already exists!");
    }

    // Otherwise create new id, convert dates from string to datetime and query the db
    let id = Uuid::new_v4();

    let start_date = match DateTime::parse_from_rfc3339(&body.start_date) {
        Ok(parsed_date) => parsed_date,
        Err(err) => {
            // handle the error
            println!("An error occurred while parsing the date: {:?}", err);
            return HttpResponse::BadRequest().json("Could not parse start_date!");
        }
    };

    let end_date = match DateTime::parse_from_rfc3339(&body.end_date) {
        Ok(parsed_date) => parsed_date,
        Err(err) => {
            // handle the error
            println!("An error occurred while parsing the date: {:?}", err);
            return HttpResponse::BadRequest().json("Could not parse end_date!");
        }
    };

    match query_add_course(&state, &id, &start_date, &end_date, &body).await {
        Ok(_) => HttpResponse::Ok().json("Course added!"),
        Err(err) => {
            if err.to_string().contains("duplicate") {
                return HttpResponse::BadRequest().json("Course already exists!");
            } else {
                if err.to_string().contains("violates foreign key") {
                    return HttpResponse::BadRequest().json("Error adding course. Please make sure that the all cities and subcategories provided really exists");
                } else {
                    return HttpResponse::InternalServerError().json("Error adding course.");
                }
            }
        }
    }
}
