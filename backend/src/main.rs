use backend::{config::Config, startup};
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use tracing::info;
use std::{env, str::FromStr};
use tracing_subscriber::{ filter::Targets, layer::SubscriberExt, util::SubscriberInitExt };


#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {

    dotenv().ok();
    color_eyre::install().expect("Failed to load color_eyre");

    let filter_layer = 
        Targets::from_str(env::var("RUST_LOG")
        .as_deref()
        .unwrap_or("info"))
        .unwrap();

    let format_layer = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(format_layer)
        .init();


    //parse our configuration from the environment
    let config = Config::parse();
    info!("Configuration loaded successfully");

    info!("Attempting to connect to the database...");
    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&config.database_url)
        .await?;
    info!("Initialized db!");

    info!("Starting database migrations...");
    sqlx::migrate!().run(&db).await?;
    info!("Database migrations completed successfully");

    startup::run(config, db).await?;
   // info!("Program Server Running...");

    Ok(())
    
}