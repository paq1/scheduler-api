use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;
use uuid::Uuid;

use crate::api::tasks::services::tasks_repository_mongo::TasksRepositoryMongo;
use crate::core::tasks::entities::task::Task;
use crate::core::tasks::services::tasks_repository::TasksRepository;
use crate::models::tasks::commands::create_task_command::CreateTaskCommand;
use crate::models::tasks::views::json_data_response::JsonDataResponse;

#[post("/tasks/commands/create", data="<create_command>")]
pub async fn create(
    tasks_repository: &State<TasksRepositoryMongo>,
    create_command: Json<CreateTaskCommand>
) -> Result<Json<JsonDataResponse>, status::Custom<Json<JsonDataResponse>>> {
    let cmd = create_command.0;
    tasks_repository
        .insert_task(
            Task {
                id: Uuid::new_v4().to_string(),
                url: cmd.url,
                http_method: cmd.http_method,
                repetition_seconds: cmd.repetition_seconds,
                state: "pending".to_string()
            }
        )
        .await
        .map(|_| Json(JsonDataResponse::new("inserted")))
        .map_err(|err| {
            status::Custom(
                Status::BadRequest,
                Json(
                    JsonDataResponse::new(err.message.as_str())
                )
            )
        })
}