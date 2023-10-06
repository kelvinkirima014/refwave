use backend::{config::Config, startup};
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use tracing::info;
use tracing_subscriber::{ layer::SubscriberExt, util::SubscriberInitExt };


#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {

    dotenv().ok();
    color_eyre::install().expect("Failed to load color_eyre");

    let format_layer = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry()
        //.with(filter_layer)
        .with(format_layer)
        .init();

    //parse our configuration from the environment
    let config = Config::parse();
  
    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&config.database_url)
        .await?;

    info!("Starting database migrations...");
    sqlx::migrate!().run(&db).await?;
  

    startup::run(config, db).await?;
   

    Ok(())
    
}