use std::sync::Arc;
use yaserde::ser::to_string;
use crate::dto::messages_request::*;
use crate::traits::ticket::MessageTrait;

pub(crate) struct MessagesRequest;

impl MessagesRequest {
    pub(crate) fn new(message: Arc<dyn MessageTrait>, user_token: &String) -> String {
        let dto = Envelope {
            body: Body {
                get_messages_request: GetMessagesRequest {
                    expressions: Expressions { message_id: message.id(), user_token: user_token.to_string() }
                }
            }
        };
        to_string(&dto).unwrap()
    }
}