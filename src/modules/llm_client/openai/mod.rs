pub mod config;
pub mod errors;
pub mod models;

use std::sync::Arc;
use tokio::sync::RwLock;

use getset::CopyGetters;
use async_openai::Client;
use isolang::Language;
use async_openai::config::OpenAIConfig;
use async_openai::error::OpenAIError;
use async_openai::types::{ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
                            CreateChatCompletionRequestArgs};


use crate::modules::llm_client::errors::TranslatorResult;
use crate::modules::llm_client::models::TranslateTask;
use crate::modules::llm_client::openai::config::OpenAIClientConfig;
use crate::ServiceConnect;
use crate::modules::llm_client::LLMClient;


#[derive(Clone, CopyGetters)]
pub struct OpenAIClient{
    options: Arc<OpenAIClientConfig>,
    client: Arc<RwLock<Client<OpenAIConfig>>>,
}

#[async_trait::async_trait]
impl ServiceConnect for OpenAIClient
    {
    type Config = OpenAIClientConfig;
    type Error = OpenAIError;
    type Client = OpenAIClient;

    async fn connect(config: &Self::Config) -> Result<Self::Client, Self::Error> {
        let address = config.address();
        let api_key = config.openai_api_key();
        let model_name = config.model_name();
        let open_ai_config = OpenAIConfig::new()
                                            .with_api_base(address)
                                            .with_api_key(api_key);

        let mut client = Client::with_config(open_ai_config);
        if *config.use_proxy(){
            let proxy_address = config.proxy_address();
            tracing::info!(proxy_address=?proxy_address, "Using proxy: {proxy_address}");
            let proxy_client = reqwest::Client::builder()
            .proxy(reqwest::Proxy::all(proxy_address)?)
            .build()?;
            client = client.with_http_client(proxy_client);
        }

        tracing::info!(address=?address, "Connection to base url: {address}");
        tracing::info!(api_key=?api_key, "API_Key: {api_key}");
        tracing::info!(model_name=?model_name, "model_name: {model_name}");
        Ok(OpenAIClient {
            options: Arc::new(config.to_owned()),
            client: Arc::new(RwLock::new(client))})
    }
}



#[async_trait::async_trait]
impl LLMClient for OpenAIClient{

    async fn translate(&self, translate_task: TranslateTask) -> TranslatorResult<String>{
        const SYSTEM_PROMPT: &str = "You are a machine translation model specialized in military and legal texts. 
                                Translate with maximum accuracy, without interpretation or alteration of meaning. 
                                Preserve terminology, structure, numbering, formatting, and the formal tone of documents. 
                                Translate military terminology according to established professional usage.
                                don't decipher the abbreviations.
                                 Do not add comments, explanations, or summaries. 
                                 If a term is ambiguous, keep the original or use the most neutral equivalent. 
                                 By default, perform translation only.";
        const MAX_TOKENS: u32 = 32_000;
        let model_name = self.options.model_name();

        let source_language = Language::from_639_1(translate_task.source_language()).unwrap().to_name();
        let target_language = Language::from_639_1(translate_task.target_language()).unwrap().to_name();

        let text = translate_task.text();
        let user_prompt = format!("Translate the following segment into {target_language}, without additional explanation.
        The {source_language} segment:
        ```
        {text}
        ```", target_language=target_language, source_language=source_language, text=text);
        println!("{}", user_prompt);
        let request = CreateChatCompletionRequestArgs::default()
                            .model(model_name)
                            .messages([
                                ChatCompletionRequestSystemMessageArgs::default()
                                    .content(SYSTEM_PROMPT)
                                    .build()?
                                    .into(),
                                ChatCompletionRequestUserMessageArgs::default()
                                    .content(user_prompt)
                                    .build()?
                                    .into(),
                            ])
                            .max_tokens(MAX_TOKENS)
                            .build()?;
        let ctx = self.client.read().await;
        let response = ctx.chat().create(request).await?;
        let transalted_response = response.choices[0].message.content.as_deref().unwrap();                       
        println!("\nResponse:\n");
        Ok(transalted_response.to_owned())
        }
}

#[cfg(test)]
mod test_open_ai {

    use crate::ServiceConnect;
    use crate::config::ServiceConfig;
    use crate::modules::llm_client::TranslateTask;

    #[tokio::test]
    async fn test_image_generation() -> Result<(), anyhow::Error> {
        let translate_task = TranslateTask::default();
        let s_config = ServiceConfig::new()?;
        let llm_client_config = s_config.llm_client();
        let mode = s_config.server().llm_mode();
        let client = mode.create_client(llm_client_config).await?;
        let result = client.translate(translate_task).await?;
        println!("{}", result);
        Ok(())
    }
}



