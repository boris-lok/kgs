#[derive(Debug, Clone)]
pub struct Config {
    pub debug: bool,
}

impl Config {
    pub fn new() -> Self {
        let debug = dotenv::var("DEBUG")
            .map(|x| x.parse::<bool>().ok())
            .ok()
            .flatten()
            .unwrap_or(true);

        Self {
            debug
        }
    }
}
