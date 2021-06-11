mod errors;
mod routes;
mod database;

use actix_web::{web, App, HttpServer};
use diesel::prelude::{SqliteConnection, Connection};
use handlebars::Handlebars;
use dotenv::dotenv;
use std::env;

#[macro_use] extern crate serde_json;
#[macro_use] extern crate diesel;

fn setup_handlebars() -> actix_web::web::Data<handlebars::Handlebars<'static>> {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_templates_directory(".html", "./static/templates")
        .expect("Could not register template dir!");

    web::Data::new(handlebars)
}

fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = env::var("REGISTRY_HOST").expect("REGISTRY_HOST must be set");
    let port = env::var("REGISTRY_PORT").expect("REGISTRY_PORT must be set");

    let connection = establish_connection();

    println!("Starting server on {}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .wrap(errors::handlers())
            .app_data(setup_handlebars())
            .service(routes::home::index)
    }).bind(("127.0.0.1", 8080))?
      .run()
      .await
}
