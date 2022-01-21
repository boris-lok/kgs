use warp::http::StatusCode;
use warp::Reply;
use warp::reply::Json;

use crate::{Environment, WebResult};
use crate::generator::json::Response;

pub async fn handler(env: Environment) -> WebResult<impl Reply> {
    let mut id_generator = env.id_generator.lock().unwrap();
    let response: Response = id_generator.next_id().into();
    let response: Json = response.into();
    Ok(warp::reply::with_status(response, StatusCode::OK))
}
