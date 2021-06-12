mod database;
mod errors;
mod routes;

use actix_web::{web, App, HttpServer};
use diesel::prelude::{Connection, SqliteConnection};
use dotenv::dotenv;
use handlebars::Handlebars;
use std::env;

#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate diesel;

fn setup_handlebars() -> actix_web::web::Data<handlebars::Handlebars<'static>> {
	let mut handlebars = Handlebars::new();

	let template_path = env::var("TEMPLATE_PATH").expect("TEMPLATE_PATH must be set");

	handlebars
		.register_templates_directory(".html", template_path)
		.expect("Could not register template dir!");

	web::Data::new(handlebars)
}

fn establish_connection() -> SqliteConnection {
	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

	SqliteConnection::establish(&database_url)
		.expect(format!("Error connecting to {}", database_url).as_str())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();

	let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
	let port = env::var("PORT")
		.unwrap_or("8080".to_string())
		.parse::<u16>()
		.expect("PORT should be a valid number");

	let connection = establish_connection();

	println!("Starting server on {}:{}", host, port);

	HttpServer::new(move || {
		App::new()
			.wrap(errors::handlers())
			.app_data(setup_handlebars())
			.service(routes::home::index)
	})
	.bind((host, port))?
	.run()
	.await
}
