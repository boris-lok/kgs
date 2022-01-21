use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use warp::Filter;

use crate::core::config::Config;
use crate::core::environment::Environment;
use crate::core::key_generator::SnowflakeGenerator;
use crate::core::recover::rejection_handler;
use crate::generator::route::routes;

type AppResult<T> = anyhow::Result<T>;
type WebResult<T> = std::result::Result<T, warp::reject::Rejection>;

mod core;
mod generator;

#[tokio::main]
async fn main() {
    let _ = dotenv::from_path("config/.env");

    tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .init();

    let config = Config::new();
    let env = Environment::new(config);

    let route = routes(env.clone())
        .with(warp::trace::request())
        .recover(rejection_handler);

    let addr = host_address().expect("Can get the host address");
    warp::serve(route).run(addr).await;
}

fn host_address() -> AppResult<SocketAddr> {
    let host_ip = dotenv::var("HOST_IP")?;
    let host_port = dotenv::var("HOST_PORT")?.parse::<u16>()?;

    let ip_address = IpAddr::from_str(&host_ip)?;
    Ok(SocketAddr::new(ip_address, host_port))
}
