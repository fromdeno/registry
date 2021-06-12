use crate::errors;

use actix_web::dev::ServiceResponse;
use actix_web::middleware::errhandlers::ErrorHandlerResponse;
use actix_web::Result;

// Error handler for a 403 Forbidden error.
pub fn handler<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
	let response = errors::get_error_response(&res, crate::constants::errors::FORBIDDEN);
	Ok(ErrorHandlerResponse::Response(
		res.into_response(response.into_body()),
	))
}
