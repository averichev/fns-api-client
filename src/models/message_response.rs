use std::sync::Arc;

use reqwest::Client;

use crate::client::error::{HttpClientError, OpenApiClientError, XmlDeserializationError};
use crate::dto::messages_response::Envelope;
use crate::models::messages_request::MessagesRequest;
use crate::traits::ticket::MessageTrait;

pub(crate) struct MessageResponse {
    http_client: Arc<Client>,
    user_token: Arc<String>,
    temp_token: Arc<String>,
}

impl MessageResponse {
    pub(crate) fn new(http_client: Arc<Client>,
                      user_token: Arc<String>,
                      temp_token: Arc<String>,
    ) -> MessageResponse {
        MessageResponse {
            http_client,
            user_token,
            temp_token,
        }
    }
    pub(crate) async fn send(&self, message: Arc<dyn MessageTrait>) -> Result<Envelope, OpenApiClientError> {
        println!("{}", message.id());
        let body = MessagesRequest::new(message, &self.user_token);

        let builder = self.http_client.post("https://openapi.nalog.ru:8090/open-api/ais3/KktService/0.1")
            .body(body.clone())
            .header("FNS-OpenApi-Token", &self.temp_token.to_string())
            .header("FNS-OpenApi-UserToken", &self.user_token.to_string());
        println!("{} {}", self.user_token, &self.temp_token);
        println!("{}", body);
        let messages_response_result = builder.send()
            .await;
        match messages_response_result {
            Ok(response) => {
                let response_test_result = response.text().await;
                match response_test_result {
                    Ok(response_text) => {
                        println!("{}", response_text);
                        let res = serde_xml_rs::from_str::<Envelope>(response_text.clone().as_str());
                        match res {
                            Ok(e) => {
                                Ok(e)
                            }
                            Err(e) => {
                                Err(OpenApiClientError::DeserializationError(XmlDeserializationError {
                                    brief: "Ошибка десериализации ответа по MessageId".to_string(),
                                    cause: e.to_string(),
                                    xml_string: response_text,
                                }))
                            }
                        }
                    }
                    Err(error) => {
                        Err(OpenApiClientError::HttpClientError(HttpClientError::new(error)))
                    }
                }
            }
            Err(error) => {
                Err(OpenApiClientError::HttpClientError(HttpClientError::new(error)))
            }
        }
    }
}

impl Envelope {
    pub(crate) fn status(&self) -> MessageStatus {
        let status_str = &self.body.get_messages_response.messages[0].result.processing_status;
        match status_str.as_ref() {
            "COMPLETED" => MessageStatus::Complete,
            "PROCESSING" => MessageStatus::Processing,
            _ => MessageStatus::Unknown,
        }
    }
}

pub enum MessageStatus {
    Complete,
    Processing,
    Unknown,
}