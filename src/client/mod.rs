pub mod error;

use reqwest::Client;
use crate::client::error::{ApiError, HttpClientError, OpenApiClientError};
use crate::models::auth_response::AuthResponse;

pub struct OpenApiClient{
    http_client: Client,
    master_token: String
}
impl OpenApiClient{
    pub fn new(master_token: &String) -> Self{
        OpenApiClient{
            http_client: Client::new(),
            master_token: master_token.to_string()
        }
    }
    pub async fn authorize(&self) -> Result<AuthResponse, OpenApiClientError> {
        let model = crate::models::auth_request::AuthRequest::new(&self.master_token);
        let auth_request = self.http_client
            .post("https://openapi.nalog.ru:8090/open-api/AuthService/0.1")
            .body(model.to_string())
            .send()
            .await;
        match auth_request {
            Ok(response) => {
                match response.text().await {
                    Ok(xml_string) => {
                        AuthResponse::new(&xml_string)
                    }
                    Err(error) => {
                        Err(OpenApiClientError::ApiError(ApiError{ message: error.to_string() }))
                    }
                }
            }
            Err(error) => {
                Err(OpenApiClientError::HttpClientError(HttpClientError{ message: error.to_string() }))
            }
        }
    }
}