use tracing::error;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

use crate::core::json::ErrorResponse;

pub async fn rejection_handler(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    let code;
    let message;

    error!("un-handle rejection: {:?}", err);

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "not founded";
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "un-handle rejection.";
    }

    let response: ErrorResponse = (code.as_u16(), message).into();

    Ok(warp::reply::with_status(warp::reply::json(&response), code))
}
