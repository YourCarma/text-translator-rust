use crate::errors::*;
use crate::server::router::models::{TextTransaltorRequest, TextTransaltorResponse};
use crate::server::router::llm_client::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title="Text Transaltor",
        version="0.9.0",
        description = "Service for text transalting with LLM"
    ),
    tags(
        (
            name = "Transaltor",
            description = "Transalting text with LLM",
        ),
    ),

    components(
        schemas(
            TextTransaltorRequest,
            Successful,
            ErrorResponse,
        ),
    ),
    paths(
       translate_text
    )
)]
pub(super) struct ApiDoc;

pub trait SwaggerExample {
    type Example;

    fn example(value: Option<&str>) -> Self::Example;
}

impl SwaggerExample for Successful {
    type Example = Self;

    fn example(value: Option<&str>) -> Self::Example {
        let msg = value.unwrap_or("Done");
        Successful::new(200, msg)
    }
}

impl SwaggerExample for ErrorResponse {
    type Example = Self;

    fn example(value: Option<&str>) -> Self::Example {
        let msg = value.unwrap_or("bad client request");
        ErrorResponse::new(400, "Bad request", msg)
    }
}
