use axum::{routing::get, Json, Router};
use uuid::Uuid;

/**
responsible for the management of all runners
*/

pub trait MapRunner {
    fn map_runner_routes(self) -> Self;
}

#[derive(serde::Serialize)]
struct Runner {
    id: Uuid,
}

async fn get_runners() -> Json<Vec<Runner>> {
    vec![Runner { id: Uuid::now_v7() }].into()
}

impl MapRunner for Router {
    fn map_runner_routes(self) -> Router {
        self.route("/runners", get(get_runners))
    }
}
