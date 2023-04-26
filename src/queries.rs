use crate::{
    models::{
        api::{
            CreateCourseRequest,
        },
        db::{
            Course,
        },
    },
    AppState,
};
use ::chrono::{DateTime, FixedOffset};
use actix_web::web::{Data, Json};
use sqlx::{self, types::chrono::Utc};
use uuid::Uuid;

pub async fn query_add_course(
    state: &Data<AppState>,
    id: &Uuid,
    start_date: &DateTime<FixedOffset>,
    end_date: &DateTime<FixedOffset>,
    course_details: &Json<CreateCourseRequest>,
) -> Result<(), sqlx::Error> {
    let mut tx = state.db.begin().await?;

    let result_create_course = sqlx::query("INSERT INTO db.courses (id, course_name, course_description, start_date, end_date, csn_entitled, max_seats, image, days, hours, price, sessions, visible) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13 )")
            .bind(&id)
            .bind(&course_details.course_name.to_string())
            .bind(&course_details.course_description.to_string())
            .bind(&start_date)
            .bind(&end_date)
            .bind(&course_details.csn_entitled)
            .bind(&course_details.max_seats)
            .bind(&course_details.image.to_string())
            .bind(&course_details.days.to_string())
            .bind(&course_details.hours.to_string())
            .bind(&course_details.price)
            .bind(&course_details.sessions)
            .bind(&course_details.visible)
            .execute(&mut tx)
            .await?
            .rows_affected();

    if result_create_course == 0 {
        return tx.rollback().await;
    }

    if !&course_details.city_ids.is_empty() {
        let mut cities_added: u64 = 0;
        for city_id in &course_details.city_ids {
            let result = sqlx::query(
                "INSERT INTO db.course_location (course_id, location_id) VALUES ($1, $2)",
            )
            .bind(id)
            .bind(city_id)
            .execute(&mut tx)
            .await?
            .rows_affected();
            cities_added += result
        }

        if cities_added == 0 {
            return tx.rollback().await;
        }
    }

    if !&course_details.subcategory_ids.is_empty() {
        let mut subcategories_added: u64 = 0;

        for subcategory_id in &course_details.subcategory_ids {
            let result = sqlx::query(
                "INSERT INTO db.course_categories (course_id, category_id) VALUES ($1, $2)",
            )
            .bind(id)
            .bind(subcategory_id)
            .execute(&mut tx)
            .await?
            .rows_affected();
            subcategories_added += result
        }

        if subcategories_added == 0 {
            return tx.rollback().await;
        }
    }

    let result = tx.commit().await;

    return result;
}

pub async fn query_get_course_by_name(
    state: &Data<AppState>,
    name: String,
) -> Result<Course, sqlx::Error> {
    let result = sqlx::query_as::<_, Course>("SELECT * FROM db.courses WHERE course_name = $1")
        .bind(name)
        .fetch_one(&state.db)
        .await;

    return result;
}
