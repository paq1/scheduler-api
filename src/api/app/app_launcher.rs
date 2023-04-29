use std::thread;
use std::time::Duration;
use clokwerk::{Scheduler, TimeUnits};
use rocket::{Build, Rocket, routes};

use crate::api::app::cors::CORS;

use crate::api::tasks::routes::task_read_router::hello;
use crate::models::tasks::errors::custom::CustomError;

pub struct AppLauncher;

impl AppLauncher {
    pub async fn launch_rocket() -> Result<Rocket<Build>, CustomError> {
        Ok(
            rocket::build()
                .attach(CORS)
                .mount(
                    "/",
                    routes![
                        hello
                    ]
                )
        )
    }
}