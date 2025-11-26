use getset::Getters;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::modules::llm_client::models::{TranslateTask};

#[derive(Serialize, Deserialize, Getters, ToSchema)]
#[getset(get = "pub")]
pub struct TextTransaltorRequest {
    #[serde(flatten)]
    translate_task: TranslateTask,
}

#[derive(Serialize, Deserialize, Getters, ToSchema)]
#[getset(get = "pub")]
pub struct TextTransaltorResponse {
    text: String,
}

impl TextTransaltorResponse {
    pub fn new(text: String) -> Self{
        Self { text: text }
    }
}


