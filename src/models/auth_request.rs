use yaserde::ser::to_string;
use crate::dto::auth_request::{Envelope, Body, GetMessageRequest, Message,
                               AuthRequest as AuthRequestDto, AuthAppInfo};

pub(crate) struct AuthRequest {
    dto: Envelope,
}

impl AuthRequest {
    pub(crate) fn new(master_token: &String) -> Self {
        AuthRequest {
            dto: Envelope {
                body: Body {
                    get_message_request: GetMessageRequest {
                        message: Message {
                            auth_request: AuthRequestDto {
                                auth_app_info: AuthAppInfo {
                                    master_token: master_token.clone()
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    pub(crate) fn to_string(&self) -> String {
        let xml_string = to_string(&self.dto).unwrap();
        xml_string
    }
}
