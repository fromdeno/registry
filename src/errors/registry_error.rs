use actix_web::{dev::HttpResponseBuilder, error, http::header, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum RegistryError {
	#[display(fmt = "Page not found!")]
	NotFound {},
}

impl error::ResponseError for RegistryError {
	fn error_response(&self) -> HttpResponse {
		HttpResponseBuilder::new(self.status_code())
			.set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
			.body(self.to_string())
	}
	fn status_code(&self) -> StatusCode {
		match *self {
			RegistryError::NotFound { .. } => StatusCode::NOT_FOUND,
		}
	}
}
