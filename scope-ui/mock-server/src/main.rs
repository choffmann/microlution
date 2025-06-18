use std::sync::Mutex;

use actix_web::{App, HttpResponse, HttpServer, get, post, web};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct Axis {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Serialize, Deserialize)]
struct MoveStageRequest {
    x: i32,
    y: i32,
    z: i32,
    absolute: bool,
}

#[derive(Serialize, Deserialize)]
struct MoveStageResponse {
    input: MoveStageRequest,
    status: String,
}

struct AppState {
    stage: Mutex<Axis>,
}

impl Default for AppState {
    fn default() -> Self {
        let stage_axis = Axis {
            x: 320,
            y: 3229,
            z: 3298,
        };

        Self {
            stage: Mutex::new(stage_axis),
        }
    }
}

#[post("/api/v2/actions/stage/move")]
async fn move_stage(data: web::Data<AppState>, req: web::Json<MoveStageRequest>) -> HttpResponse {
    let mut state_axis = data.stage.lock().unwrap();
    state_axis.x = state_axis.x + req.x;
    state_axis.y = state_axis.y + req.y;
    state_axis.z = state_axis.z + req.z;

    let body = MoveStageResponse {
        input: MoveStageRequest {
            x: state_axis.x,
            y: state_axis.y,
            z: state_axis.z,
            absolute: req.absolute,
        },
        status: String::from("pending"),
    };
    HttpResponse::Ok().json(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind = ("127.0.0.1", 8080);

    env_logger::init();
    info!("starting server on {}:{}", bind.0, bind.1);

    let state = web::Data::new(AppState::default());
    HttpServer::new(move || App::new().app_data(state.clone()).service(move_stage))
        .bind(bind)?
        .run()
        .await
}
