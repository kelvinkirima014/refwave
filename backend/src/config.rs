//use std::env;
use dotenv::dotenv;
use clap::Parser;

///The configuration parameters for the application.
/// 
/// These parameters should be pulled from environment variables
/// But you could also pass them via the CLI especially in a production 
/// deployment setting.

#[derive(clap::Parser, Clone)]
pub struct Config {
    /// The connection URL for the Postgres database this application should use.
    #[clap(long, env)]
    pub database_url: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        Config::parse()
    }
}
