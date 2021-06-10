use crate::errors;

use actix_web::middleware::errhandlers::{ErrorHandlerResponse};
use actix_web::dev::ServiceResponse;
use actix_web::Result;


// Error handler for a 404 Page not found error.
pub fn handler<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let response = errors::get_error_response(&res, "Page not found");
    Ok(ErrorHandlerResponse::Response(
        res.into_response(response.into_body()),
    ))
}
