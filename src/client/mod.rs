use std::sync::Arc;

use chrono::{NaiveDateTime, ParseError};
use reqwest::Client;
use tokio::time::{Duration, sleep};
use yaserde::ser::to_string;
use log::debug;
use serde_json::from_str;

use crate::client::error::{FnsApiError, HttpClientError, OpenApiClientError};
use crate::dto::ticket_request;
use crate::dto::ticket_response::Envelope;
use crate::models::auth_response::{AuthResponse, AuthResponseResult, AuthResponseToken};
use crate::models::message_response::{MessageResponse, MessageStatus};
use crate::models::ticket::TicketResponse;
use crate::traits::check_query::CheckQueryTrait;
use crate::traits::ticket::{Ticket, TicketResponseResult, TicketResponseTrait, TicketTrait};

pub mod error;

pub struct OpenApiClient {
    http_client: Client,
    master_token: String,
    user_token: String,
    temp_token: Option<AuthResponseToken>,
}

impl OpenApiClient {
    pub fn new(master_token: &String, user_token: &String) -> Self {
        OpenApiClient {
            http_client: Client::new(),
            master_token: master_token.to_string(),
            user_token: user_token.to_string(),
            temp_token: None,
        }
    }
    pub async fn authorize(&mut self) -> Result<AuthResponse, OpenApiClientError> {
        debug!("Авторизация");
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
                                        self.temp_token = Some(token);
                                        debug!("Токен получен");
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
    pub async fn get_ticket_with_retry(&self, check_query: impl CheckQueryTrait + Clone) -> Result<Arc<crate::dto::ticket::Ticket>, OpenApiClientError> {
        let mut attempt = 0;
        let max_attempts = 5;
        let base_delay = Duration::from_secs(1);

        while attempt < max_attempts {
            match self.get_ticket(check_query.clone()).await {
                Ok(ticket) => {
                    debug!("Ответ {:?}", ticket);
                    let status = match ticket.status() {
                        Some(status) => status,
                        None => return Err(OpenApiClientError::Error(String::from("Ошибка получения статуса"))),
                    };
                    debug!("Статус {:?}", status);
                    match status {
                        MessageStatus::Complete => {
                            let json_string = ticket.ticket_json();
                            debug!("{}", json_string.clone());
                            let ticket_json = from_str::<crate::dto::ticket::Ticket>(json_string.as_str()).unwrap();
                            return Ok(Arc::new(ticket_json))
                        }
                        MessageStatus::Processing => {
                            let delay = base_delay * 2u32.pow(attempt);
                            println!("Retry attempt {}: waiting for {:?} before next retry", attempt + 1, delay);
                            sleep(delay).await;
                            attempt += 1;
                        }
                        MessageStatus::Unknown => {
                            return Err(OpenApiClientError::Error("Неизвестный статус сообщения".to_string()))
                        }
                    }
                },
                Err(e) => {
                    return Err(e)
                }
            }
        }
        Err(OpenApiClientError::Error(String::from("Failed after max retry attempts")))
    }
    pub async fn get_ticket(&self, check_query: impl CheckQueryTrait) -> Result<crate::dto::messages_response::Envelope, OpenApiClientError> {
        debug!("Получение чека");
        let ticket_response = self.get_ticket_response(check_query).await;
        let temp_token = match &self.temp_token {
            Some(token) => token,
            None => return Err(OpenApiClientError::Error(String::from("Отсутствует FNS-OpenApi-UserToken"))),
        };
        match ticket_response {
            Ok(ticket_response) => {
                match ticket_response.result() {
                    TicketResponseResult::Ok(message) => {
                        debug!("Сообщение {:?}", message);
                        let response = MessageResponse::new(Arc::new(self.http_client.clone()), Arc::new(self.user_token.clone()), Arc::new(temp_token.clone()))
                            .send(message).await;
                        response
                    }
                    TicketResponseResult::Err(err) => {
                        Err(OpenApiClientError::FnsApiError(FnsApiError { message: err.message() }))
                    }
                }
            }
            Err(error) => {
                Err(error)
            }
        }
    }
    async fn get_ticket_response(&self, check_query: impl CheckQueryTrait) -> Result<Arc<dyn TicketResponseTrait>, OpenApiClientError> {
        match &self.temp_token {
            None => {
                Err(OpenApiClientError::Error(String::from("Отсутствует FNS-OpenApi-UserToken")))
            }
            Some(temp_token) => {
                fn convert_datetime(datetime: String) -> Result<String, ParseError> {
                    let parsed_date = NaiveDateTime::parse_from_str(datetime.as_str(), "%Y%m%dT%H%M")?;
                    Ok(parsed_date.format("%Y-%m-%dT%H:%M:%S").to_string())
                }

                let ticket_request = ticket_request::Envelope {
                    body: ticket_request::Body {
                        send_message_request: ticket_request::SendMessageRequest {
                            message: ticket_request::Message {
                                get_ticket_request: ticket_request::GetTicketRequest {
                                    get_ticket_info: ticket_request::GetTicketInfo {
                                        sum: check_query.s(),
                                        date: convert_datetime(check_query.t()).unwrap(),
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
                let body = to_string(&ticket_request).unwrap();
                debug!("Отправка запроса на получение чека");
                println!("{}", body.clone());
                let ticket_info_query = self.http_client.post("https://openapi.nalog.ru:8090/open-api/ais3/KktService/0.1")
                    .body(body)
                    .header("FNS-OpenApi-Token", &temp_token.value)
                    .header("FNS-OpenApi-UserToken", &self.user_token);
                let ticket_info_response = ticket_info_query.send()
                    .await;
                match ticket_info_response {
                    Ok(response) => {
                        match response.text().await {
                            Ok(text) => {
                                let deserialize = crate::models::serde::Serde::from_xml::<Envelope>(&text);
                                match deserialize {
                                    Ok(dto) => {
                                        debug!("Ответ получен");
                                        debug!("{:?}", dto);
                                        debug!("{}", text);
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
    }
}