mod forbidden;
mod not_found;

use actix_http::{body::Body, Response};
use actix_web::dev::ServiceResponse;
use actix_web::http::StatusCode;
use actix_web::middleware::errhandlers::ErrorHandlers;
use actix_web::web;
use handlebars::Handlebars;

pub fn handlers() -> ErrorHandlers<Body> {
	ErrorHandlers::new()
		.handler(StatusCode::NOT_FOUND, not_found::handler)
		.handler(StatusCode::FORBIDDEN, forbidden::handler)
	// and so on...
}

// Generic error handler
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> Response<Body> {
	let request = res.request();

	// Provide a fallback to a simple plain text response in case an error occurs during the
	// rendering of the error page.
	let fallback = |e: &str| {
		Response::build(res.status())
			.content_type("text/plain")
			.body(e.to_string())
	};

	let hb = request
		.app_data::<web::Data<Handlebars>>()
		.map(|t| t.get_ref());
	match hb {
		Some(hb) => {
			let data = json!({
				"code": res.status().as_str(),
				"msg": error,
			});
			let body = hb.render("error", &data);

			match body {
				Ok(body) => Response::build(res.status())
					.content_type("text/html")
					.body(body),
				Err(_) => fallback(error),
			}
		}
		None => fallback(error),
	}
}
