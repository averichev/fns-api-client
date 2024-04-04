use std::sync::Arc;

use crate::client::error::{FnsApiError, OpenApiClientError};
use crate::dto::ticket_response::{Body, Envelope, SendMessageResponse};
use crate::traits::ticket::{TicketRequestErrorTrait, TicketResponseResult, TicketResponseTrait, MessageTrait};

#[derive(Clone)]
pub(crate) struct TicketResponse {
    result: TicketResponseResult,
}

impl TicketResponse {
    pub(crate) fn new(response_dto: Envelope) -> Result<Arc<dyn TicketResponseTrait>, OpenApiClientError> {
        match response_dto.body {
            Body::Fault(fault) => {
                Err(OpenApiClientError::FnsApiError(FnsApiError { message: fault.faultstring }))
            }
            Body::SendMessageResponse(t) => {
                Ok(TicketResponse::ok(Message::new(t)))
            }
        }
    }
    fn ok(ticket: Arc<dyn MessageTrait>) -> Arc<dyn TicketResponseTrait> {
        Arc::new(TicketResponse { result: TicketResponseResult::Ok(ticket) })
    }
}

impl TicketResponseTrait for TicketResponse {
    fn result(&self) -> TicketResponseResult {
        self.result.clone()
    }
}

struct TicketRequestError {
    message: String,
}

impl TicketRequestError {
    fn new(message: String) -> Box<dyn TicketRequestErrorTrait> {
        Box::new(TicketRequestError {
            message
        })
    }
}

impl TicketRequestErrorTrait for TicketRequestError {
    fn message(&self) -> String {
        self.message.clone()
    }
}

#[derive(Clone)]
pub(super) struct Message{
    id: String
}

impl MessageTrait for Message {
    fn id(&self) -> String {
        self.id.clone()
    }
}

impl Message {
    fn new(response: SendMessageResponse) -> Arc<dyn MessageTrait> {
        Arc::new(Message { id: response.message_id })
    }
}