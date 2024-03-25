use yaserde::de::from_str;
use crate::client::error::{OpenApiClientError, XmlDeserializationError};
use crate::dto::auth_response::{Body, Envelope};

#[derive(Clone)]
pub struct AuthResponse {
    pub result: AuthResponseResult,
}

impl AuthResponse {
    pub(crate) fn new(xml_string: &String) -> Result<AuthResponse, OpenApiClientError> {
        let auth_response = from_str::<Envelope>(xml_string);
        match auth_response {
            Ok(ok) => {
                let body = ok.body;
                let result = match body {
                    Body::Fault(fault) => {
                        AuthResponseResult::Error(AuthError{ message: fault.faultstring })
                    }
                    Body::GetMessageResponse(response) => {
                        AuthResponseResult::Ok(AuthResponseToken { value: response.message.auth_response.auth_app_info.token })
                    }
                };
                Ok(AuthResponse { result })
            }
            Err(message) => {
                return Err(OpenApiClientError::DeserializationError(
                    XmlDeserializationError{
                        cause: message,
                        brief: "Ошибка десериализации ответа при авторизации".to_string(),
                        xml_string: xml_string.to_string()
                    }));
            }
        }
    }
}

#[derive(Clone)]
pub enum AuthResponseResult {
    Ok(AuthResponseToken),
    Error(AuthError),
}

#[derive(Clone)]
pub struct AuthError{
    pub message: String
}

#[derive(Clone)]
pub struct AuthResponseToken {
    pub value: String,
}