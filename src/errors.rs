use actix_web::{error, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::error::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CanPiAppError {
    #[error("ActixError: {0}")]
    ActixError(String),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Template Error: {0}")]
    TeraError(String),

    #[error("Other Error: {0}")]
    Other(#[from] Box<dyn Error + Send + Sync>),
}
#[derive(Debug, Serialize)]
pub struct AppErrorResponse {
    error_message: String,
}

impl CanPiAppError {
    fn error_response(&self) -> String {
        use CanPiAppError::*;

        match self {
            ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            TeraError(msg) => {
                println!("Error in rendering the template {:?}", msg);
                msg.into()
            }
            NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
            Other(err) => {
                let msg = format!("{}", err);
                println!("Internal Error: {}", msg);
                msg
            }
        }
    }
}

impl error::ResponseError for CanPiAppError {
    fn status_code(&self) -> StatusCode {
        use CanPiAppError::*;

        match self {
            ActixError(_) | TeraError(_) | Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
            NotFound(_msg) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl From<actix_web::error::Error> for CanPiAppError {
    fn from(err: actix_web::error::Error) -> Self {
        CanPiAppError::ActixError(err.to_string())
    }
}

impl From<std::io::Error> for CanPiAppError {
    fn from(err: std::io::Error) -> CanPiAppError {
        CanPiAppError::Other(Box::new(err))
    }
}

impl From<strfmt::FmtError> for CanPiAppError {
    fn from(_err: strfmt::FmtError) -> CanPiAppError {
        CanPiAppError::TeraError("strfmt error".to_string())
    }
}
