use crate::constants;
use crate::database::models::config::util;
use crate::errors::registry_error;
use actix_web::{get, web, HttpResponse};
use diesel::SqliteConnection;
use handlebars::Handlebars;

#[get("/ctl/register")]
async fn index(
	conn: web::Data<SqliteConnection>,
	hb: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, registry_error::RegistryError> {
	if util::get_config(&conn, constants::configs::SIGNUPS).unwrap_or("false".to_string()) == "true"
	{
		let data = json!({
			"what": "account"
		});

		let body = hb.render("register", &data).unwrap();

		Ok(HttpResponse::Ok().body(body))
	} else {
		Ok(HttpResponse::NotFound().finish())
		// Err(registry_error::RegistryError::NotFound {})
	}
}
