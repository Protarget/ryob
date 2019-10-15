use actix_web::error::ResponseError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;

#[derive(Debug)]
pub struct RyobError(pub StatusCode, pub String);

impl RyobError {
    pub fn from_display<T: std::fmt::Display>(code: StatusCode, value: T) -> RyobError {
        RyobError(code, format!("{}", value))
    }
}

impl std::fmt::Display for RyobError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.1)
    }
}

impl ResponseError for RyobError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.0).finish()
    }
}

impl From<r2d2::Error> for RyobError {
    fn from(error: r2d2::Error) -> RyobError {
        RyobError::from_display(StatusCode::INTERNAL_SERVER_ERROR, error)
    }
}

impl From<handlebars::RenderError> for RyobError {
    fn from(error: handlebars::RenderError) -> RyobError {
        RyobError::from_display(StatusCode::INTERNAL_SERVER_ERROR, error)
    }
}
