use warp::{Filter, Reply};
use warp::filters::BoxedFilter;

use crate::core::middlewares::with_env::with_env;
use crate::Environment;
use crate::generator::handlers::v1::handler;

pub fn routes(env: Environment) -> BoxedFilter<(impl Reply,)> {
    let route = warp::path!("api" / "v1" / "id")
        .and(warp::path::end())
        .and(warp::get())
        .and(with_env(env))
        .and_then(handler);

    route.boxed()
}
