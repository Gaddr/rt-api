use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateCourseRequest {
    pub course_name: String,
    pub course_description: String,
    pub start_date: String,
    pub end_date: String,
    pub csn_entitled: bool,
    pub max_seats: i32,
    pub image: String,
    pub days: String,
    pub hours: String,
    pub price: i32,
    pub sessions: i32,
    pub visible: bool,
    pub city_ids: Vec<Uuid>,
    pub subcategory_ids: Vec<Uuid>,
}