use std::sync::{Arc, Mutex};

use crate::{Config, SnowflakeGenerator};

#[derive(Clone)]
pub struct Environment {
    pub config: Arc<Config>,
    pub id_generator: Arc<Mutex<SnowflakeGenerator>>,
}

impl Environment {
    pub fn new(config: Config) -> Self {
        let worker_id = dotenv::var("WORKER_ID")
            .expect("Can read worker id from env.")
            .parse::<u8>()
            .expect("Can parse worker id to u16");

        let data_center_id = dotenv::var("DATA_CENTER_ID")
            .expect("Can read data center id from env.")
            .parse::<u8>()
            .expect("Can parse data center id to u16");

        let id_generator = SnowflakeGenerator::new(worker_id, data_center_id);

        Self {
            config: Arc::new(config),
            id_generator: Arc::new(Mutex::new(id_generator)),
        }
    }
}
