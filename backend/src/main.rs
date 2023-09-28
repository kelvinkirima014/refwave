use backend::{config::Config, startup};
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use std::{env, str::FromStr};
use tracing_subscriber::{ filter::Targets, layer::SubscriberExt, util::SubscriberInitExt };


#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {

    dotenv().ok();
    color_eyre::install().unwrap();

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

    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&config.database_url)
        .await?;



    sqlx::migrate!().run(&db).await?;


    // if let Ok(_) = run(config, db).await {
    //     info!("Server is running successfully!")
    // }

    startup::run(config, db).await?;
    

    // match run().await {
    //     Ok(_) => println!("Server is running successfully!"),
    //     Err(e) => eprintln!("Server encounted an error: {:?}", e) 
    // }    


    Ok(())
    
}