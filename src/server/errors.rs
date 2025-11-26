use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

use crate::server::swagger::SwaggerExample;
use crate::modules::llm_client::errors::TranslatorErrors;

pub type ServerResult<T> = Result<T, ServerError>;

#[derive(Debug, Error, Serialize, ToSchema)]
pub enum ServerError {
    #[error("not found error: {0}")]
    NotFound(String),
    #[error("Provider is unavailable: {0}")]
    ServiceUnavailable(String),
    #[error("Unauthorized request: {0}")]
    Unauthorized(String),
    #[error("Deserialize Error {0}")]
    DeserializeError(String),
    #[error("API key has no credits: {0}")]
    NoCredits(String),
    #[error("Model requires a moderation {0}")]
    ModelModerationError(String),
    #[error("User is limited: {0}")]
    RateLimited(String),
    #[error("Invalid Model Response: {0}")]
    InvalidResponse(String),
    #[error("IO Error")]
    IOError(String),
    #[error("Timeout Error")]
    Timeout(String),
    #[error("Bad request")]
    BadRequest(String),
    #[error("Internal server error: {0}")]
    InternalError(String),
    #[error("Serde error: {0}")]
    SerdeError(String),
    #[error("Reqeust error: {0}")]
    RequestError(String),
}

impl ServerError {
    pub fn status_code(&self) -> (String, StatusCode) {
        match self {
            ServerError::NotFound(msg) => (msg.to_owned(), StatusCode::NOT_FOUND),
            ServerError::BadRequest(msg) => (msg.to_owned(), StatusCode::BAD_REQUEST),
            ServerError::DeserializeError(msg) => (msg.to_owned(), StatusCode::BAD_GATEWAY),
            ServerError::IOError(msg) => (msg.to_owned(), StatusCode::NO_CONTENT),
            ServerError::RequestError(msg) => (msg.to_owned(), StatusCode::BAD_GATEWAY),
            ServerError::InvalidResponse(msg) => (msg.to_owned(), StatusCode::NO_CONTENT),
            ServerError::NoCredits(msg) => (msg.to_owned(), StatusCode::PAYMENT_REQUIRED),
            ServerError::RateLimited(msg) => (msg.to_owned(), StatusCode::TOO_MANY_REQUESTS),
            ServerError::Timeout(msg) => (msg.to_owned(), StatusCode::REQUEST_TIMEOUT),
            ServerError::ServiceUnavailable(msg) => (msg.to_owned(), StatusCode::SERVICE_UNAVAILABLE),
            ServerError::Unauthorized(msg) => (msg.to_owned(), StatusCode::UNAUTHORIZED),
            ServerError::InternalError(msg) => (msg.to_owned(), StatusCode::INTERNAL_SERVER_ERROR),
            ServerError::ModelModerationError(msg) => (msg.to_owned(), StatusCode::FORBIDDEN),
            ServerError::SerdeError(msg) => (msg.to_owned(), StatusCode::UNPROCESSABLE_ENTITY),
        }
    }
}

impl From<TranslatorErrors> for ServerError {
    fn from(err: TranslatorErrors) -> Self {
        tracing::error!("Error: {err}", err=err.to_string());
        match err {
            TranslatorErrors::BadRequest(err) => ServerError::BadRequest(err.to_string()),
            TranslatorErrors::DeserializeError(_err) => ServerError::DeserializeError("No image on response of model. Try again.".to_string()),
            TranslatorErrors::IOError(_err) => ServerError::IOError("File saving or sending error.".to_string()),
            TranslatorErrors::RequestError(_err) => ServerError::RequestError("Request Error".to_string()),
            TranslatorErrors::InvalidResponse(_err) => ServerError::InvalidResponse("Invalid Response from generate API. See the logs".to_string()),
            TranslatorErrors::ModelModerationError(_err) => ServerError::ModelModerationError("Model API is on moderation.Try another model".to_string()),
            TranslatorErrors::NoCredits(_err) => ServerError::NoCredits("You have not credits on API".to_string()),
            TranslatorErrors::RateLimited(_err) => ServerError::RateLimited("Too many requests for API".to_string()),
            TranslatorErrors::ServiceUnavailable(_err) => ServerError::ServiceUnavailable("Model provider is unavailable".to_string()),
            TranslatorErrors::Timeout(_err) => ServerError::Timeout("API request to model timeout".to_string()),
            TranslatorErrors::Unauthorized(_err) => ServerError::Unauthorized("Unauthorized to API".to_string()),
            TranslatorErrors::AnotherError(_err) => ServerError::InternalError("Internal server error".to_string()),
        }
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (msg, status) = self.status_code();
        let mut resp = Json(ErrorResponse {
            message: msg.to_string(),
        })
        .into_response();

        *resp.status_mut() = status;
        resp
    }
}

impl SwaggerExample for ServerError {
    type Example = Self;

    fn example(value: Option<&str>) -> Self::Example {
        match value {
            None => ServerError::ServiceUnavailable("Service unavailable".to_owned()),
            Some(msg) => ServerError::InternalError(msg.to_owned()),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct Success {
    status: u16,
    message: String,
}

impl Default for Success {
    fn default() -> Self {
        Success {
            status: 200,
            message: "Ok".to_string(),
        }
    }
}

impl SwaggerExample for Success {
    type Example = Self;

    fn example(_value: Option<&str>) -> Self::Example {
        Success::default()
    }
}
