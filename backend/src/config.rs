///The configuration parameters for the application
/// 
/// These parameters should be pulled from environment variables
/// But you could also pass them via the CLI especially in a production 
/// deployment setting.

#[derive(clap::Parser)]
pub struct Config {
    /// The connection URL for the Postgres database this application should use.
    #[clap(long, env)]
    pub database_url: String,
}