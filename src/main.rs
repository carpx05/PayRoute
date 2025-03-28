use actix_web::{App, HttpServer, web, HttpResponse};
use payroute_rs::router::init_routes;
use std::sync::Arc;
use tokio::sync::RwLock;
use payroute_rs::config::Rule;

type SharedRules = Arc<RwLock<Vec<Rule>>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let shared_rules: SharedRules = Arc::new(RwLock::new(vec![]));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shared_rules.clone()))
            .configure(init_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}