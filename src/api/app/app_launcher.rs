use rocket::{Build, Rocket, routes};

use crate::api::components::cors::CORS;

pub struct AppLauncher;

impl AppLauncher {
    pub async fn launch_rocket() -> Result<Rocket<Build>, CustomError> {
        Ok(
            rocket::build()
                .attach(CORS)
                .mount(
                    "/",
                    routes![

                            ],
                )
        )
    }
}
