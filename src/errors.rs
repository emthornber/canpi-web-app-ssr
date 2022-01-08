use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum CanPiAppError {
    ActixError(String),
    NotFound(String),
    TeraError(String),
}
#[derive(Debug, Serialize)]
pub struct AppErrorResponse {
    error_message: String,
}
impl std::error::Error for CanPiAppError {}

impl CanPiAppError {
    fn error_response(&self) -> String {
        match self {
            CanPiAppError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            CanPiAppError::TeraError(msg) => {
                println!("Error in rendering the template {:?}", msg);
                msg.into()
            }
            CanPiAppError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for CanPiAppError {
    fn status_code(&self) -> StatusCode {
        match self {
            CanPiAppError::ActixError(_msg)
            | CanPiAppError::TeraError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            CanPiAppError::NotFound(_msg) => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl fmt::Display for CanPiAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for CanPiAppError {
    fn from(err: actix_web::error::Error) -> Self {
        CanPiAppError::ActixError(err.to_string())
    }
}
