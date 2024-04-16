use std::sync::Arc;
use log::debug;

use reqwest::Client;
use tokio::time::{Duration, sleep};

use crate::client::error::{HttpClientError, OpenApiClientError, XmlDeserializationError};
use crate::dto::messages_response::Envelope;
use crate::models::auth_response::AuthResponseToken;
use crate::models::messages_request::MessagesRequest;
use crate::traits::ticket::MessageTrait;

pub(crate) struct MessageResponse {
    http_client: Arc<Client>,
    user_token: Arc<String>,
    temp_token: Arc<AuthResponseToken>,
}

impl MessageResponse {
    pub(crate) fn new(http_client: Arc<Client>,
                      user_token: Arc<String>,
                      temp_token: Arc<AuthResponseToken>,
    ) -> MessageResponse {
        MessageResponse {
            http_client,
            user_token,
            temp_token,
        }
    }
    pub(crate) async fn send(&self, message: Arc<dyn MessageTrait>) -> Result<Envelope, OpenApiClientError> {
        sleep(Duration::from_secs_f32(0.5)).await;
        debug!("{:#?}", message);
        let body = MessagesRequest::new(message, &self.user_token);
        debug!("{}", body);
        let builder = self.http_client.post("https://openapi.nalog.ru:8090/open-api/ais3/KktService/0.1")
            .body(body.clone())
            .header("content-type", "text/xml;charset=utf-8")
            .header("SOAPAction", "urn:GetMessagesRequest")
            .header("FNS-OpenApi-Token", &self.temp_token.value)
            .header("FNS-OpenApi-UserToken", &self.user_token.to_string());
        debug!("{} {}", self.user_token, &self.temp_token.value);
        debug!("{}", body);
        let messages_response_result = builder.send()
            .await;
        match messages_response_result {
            Ok(response) => {
                let response_test_result = response.text().await;
                match response_test_result {
                    Ok(response_text) => {
                        debug!("{}", response_text);
                        let res = serde_xml_rs::from_str::<Envelope>(response_text.clone().as_str());
                        debug!("{:?}", res);
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
    pub(crate) fn status(&self) -> Option<MessageStatus> {
        let res = &self.body.get_messages_response.messages[0].result;
        match res {
            None => {
                None
            }
            Some(s) => {
                let status_str = &s.processing_status;
                let n = match status_str.as_str() {
                    "COMPLETED" => MessageStatus::Complete,
                    "PROCESSING" => MessageStatus::Processing,
                    _ => MessageStatus::Unknown,
                };
                Some(n)
            }
        }

    }
}

#[derive(Debug)]
pub enum MessageStatus {
    Complete,
    Processing,
    Unknown,
}