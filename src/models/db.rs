use chrono::FixedOffset;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::DateTime;
use sqlx::{self, types::chrono::Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize)]
pub struct Course {
    pub id: Uuid,
    pub course_name: String,
    pub course_description: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub csn_entitled: bool,
    pub max_seats: i32,
    pub image: String,
    pub days: String,
    pub hours: String,
    pub price: i32,
    pub sessions: i32,
    pub visible: bool,
    pub district_ids: Vec<Option<Uuid>>,
    pub city_ids: Vec<Option<Uuid>>,
    pub category_ids: Vec<Option<Uuid>>,
    pub subcategory_ids: Vec<Option<Uuid>>, // pub subcategory_ids: Vec<Uuid>,
                                            // pub booking_count: i64,
}
