use axum::http::HeaderName;
use axum::routing::get;
use hyper::Method;
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::cors::{ CorsLayer, Any };
use std::net::SocketAddr;
use axum::{
    Router,
    Extension
};
use tracing::info;

use crate::routes::users::router;
use crate::config::Config;
use crate::routes::server_sent::sse_handler;


///The core type through which handler functions can access common API state.
/// 
/// This can be accessed by adding a parameter `Extension<ApiContext>` to a handler function's
/// parameters

#[derive(Clone)]
pub struct ApiContext {
    pub config: Config,//Arc<Config>,
    pub db: PgPool,
}


pub fn api_router() -> Router {
    router()
    .route("/sse", get(sse_handler))
}

pub async fn run(config: Config, db: PgPool) -> color_eyre::Result<(), anyhow::Error> {

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::POST, Method::GET, Method::PUT, Method::DELETE])
        .allow_headers(vec![
            HeaderName::from_static("content-type"),
            HeaderName::from_static("authorization"),
    ]);

    let app = api_router().layer(
        ServiceBuilder::new()
        .layer(cors)
        .layer(Extension(ApiContext {
            config,//Arc::new(config),
            db,
        }))
        );

    let addr: SocketAddr = ([127, 0, 0, 1], 8080).into();
    info!("Server listening on port: http://{:?}", addr);

    // run it with hyper on localhost:8080
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
    
    
}