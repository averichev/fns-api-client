use yaserde::de::from_str;
use crate::dto::auth_response::Body;

pub(crate) struct AuthResponse {
    pub result: AuthResponseResult
}

impl AuthResponse {
    pub(crate) fn new(xml_string: &String) -> AuthResponse {
        let auth_response = from_str::<crate::dto::auth_response::Envelope>(xml_string).unwrap();
        let body = auth_response.body;
        let result = match body {
            Body::Fault(fault) => {
                AuthResponseResult::Error(fault.faultstring)
            }
            Body::GetMessageResponse(response) => {
                AuthResponseResult::Ok(AuthResponseToken{ value: response.message.auth_response.auth_app_info.token })
            }
        };
        AuthResponse{result}
    }
    pub(crate) fn error(message: &String) -> AuthResponse{
        AuthResponse{ result: AuthResponseResult::Error(message.clone()) }
    }
}

pub enum AuthResponseResult{
    Ok(AuthResponseToken),
    Error(String)
}

struct AuthResponseToken{
    value: String
}