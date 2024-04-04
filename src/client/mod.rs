use std::sync::Arc;

use reqwest::Client;
use yaserde::ser::to_string;

use crate::client::error::{FnsApiError, HttpClientError, OpenApiClientError};
use crate::dto::ticket_request;
use crate::dto::ticket_response::Envelope;
use crate::models::auth_response::{AuthResponse, AuthResponseResult, AuthResponseToken};
use crate::models::ticket::TicketResponse;
use crate::traits::check_query::CheckQueryTrait;
use crate::traits::ticket::TicketResponseTrait;

pub mod error;

pub struct OpenApiClient {
    http_client: Client,
    master_token: String,
    temp_token: Option<AuthResponseToken>,
}

impl OpenApiClient {
    pub fn new(master_token: &String) -> Self {
        OpenApiClient {
            http_client: Client::new(),
            master_token: master_token.to_string(),
            temp_token: None,
        }
    }
    pub async fn authorize(&mut self) -> Result<AuthResponse, OpenApiClientError> {
        let model = crate::models::auth_request::AuthRequest::new(&self.master_token);
        let query = self.http_client
            .post("https://openapi.nalog.ru:8090/open-api/AuthService/0.1")
            .body(model.to_string());
        let auth_request = query
            .send()
            .await;
        match auth_request {
            Ok(response) => {
                match response.text().await {
                    Ok(xml_string) => {
                        let auth_response = AuthResponse::new(&xml_string);
                        match auth_response.clone() {
                            Ok(res) => {
                                match res.result {
                                    AuthResponseResult::Ok(token) => {
                                        self.temp_token = Some(token)
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        };
                        auth_response
                    }
                    Err(error) => {
                        Err(OpenApiClientError::FnsApiError(FnsApiError { message: error.to_string() }))
                    }
                }
            }
            Err(error) => {
                Err(OpenApiClientError::HttpClientError(HttpClientError { message: error.to_string() }))
            }
        }
    }
    pub async fn get_ticket(&self, check_query: impl CheckQueryTrait) -> Result<Arc<dyn TicketResponseTrait>, OpenApiClientError> {
        match &self.temp_token {
            None => {
                Err(OpenApiClientError::Error(String::from("Отсутствует FNS-OpenApi-UserToken")))
            }
            Some(temp_token) => {
                let ticket_request = ticket_request::Envelope {
                    body: ticket_request::Body {
                        send_message_request: ticket_request::SendMessageRequest {
                            message: ticket_request::Message {
                                get_ticket_request: ticket_request::GetTicketRequest {
                                    get_ticket_info: ticket_request::GetTicketInfo {
                                        sum: check_query.s(),
                                        date: check_query.t(),
                                        r#fn: check_query.r#fn(),
                                        type_operation: check_query.n(),
                                        fiscal_document_id: check_query.i(),
                                        fiscal_sign: check_query.fp(),
                                    }
                                }
                            }
                        }
                    }
                };
                println!("{}", to_string(&ticket_request).unwrap());
                let ticket_info_query = self.http_client.post("https://openapi.nalog.ru:8090/open-api/ais3/KktService/0.1")
                    .body(to_string(&ticket_request).unwrap())
                    .header("FNS-OpenApi-Token", &temp_token.value)
                    .header("FNS-OpenApi-UserToken", "test");
                let ticket_info_response = ticket_info_query.send()
                    .await;
                match ticket_info_response {
                    Ok(response) => {
                        match response.text().await {
                            Ok(text) => {
                                let deserialize = crate::models::serde::Serde::from_xml::<Envelope>(&text);
                                match deserialize {
                                    Ok(dto) => {
                                        TicketResponse::new(dto)
                                    }
                                    Err(xml_deserialization_error) => {
                                        Err(OpenApiClientError::DeserializationError(xml_deserialization_error))
                                    }
                                }
                            }
                            Err(error) => {
                                Err(OpenApiClientError::FnsApiError(FnsApiError { message: error.to_string() }))
                            }
                        }
                    }
                    Err(error) => {
                        Err(OpenApiClientError::HttpClientError(HttpClientError::new(error)))
                    }
                }
            }
        }
        // match ticket_info_response {
        //     Ok(response) => {
        //         let header = response.headers();
        //         let content_type = header[CONTENT_TYPE].to_str().unwrap();
        //         let mut headers = HeaderMap::new();
        //         headers.insert(header::CONTENT_TYPE, content_type.parse().unwrap());
        //         (StatusCode::OK, headers, response.text().await.unwrap().to_string()).into_response()
        //     }
        //     Err(e) => {
        //         (StatusCode::BAD_GATEWAY, e.to_string()).into_response()
        //     }
        // }
    }
}