use warp::Reply;

use crate::{Environment, WebResult};
use crate::generator::json::Response;

pub async fn handler(env: Environment) -> WebResult<impl Reply> {
    let mut id_generator = env.id_generator.lock().unwrap();
    let response: Response = id_generator.next_id().into();
    Ok(warp::reply::json(&response))
}
