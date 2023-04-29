use rocket::response::status;
use rocket::serde::json::Json;

use crate::models::tasks::views::json_data_response::JsonDataResponse;

#[get("/hello-world")]
pub async fn hello() -> Result<Json<JsonDataResponse>, status::Custom<Json<JsonDataResponse>>> {
    Ok(Json(JsonDataResponse::new("hello world")))
}