mod errors;
mod routes;

use actix_web::{web, App, HttpServer};
use handlebars::Handlebars;

#[macro_use]
extern crate serde_json;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();

    let handlebars_ref = web::Data::new(handlebars);

    let port = 8080;
    let host = "127.0.0.1";

    println!("Starting server on {}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .wrap(errors::handlers())
            .app_data(handlebars_ref.clone())
            .service(routes::home::index)
    }).bind(("127.0.0.1", 8080))?
      .run()
      .await
}
