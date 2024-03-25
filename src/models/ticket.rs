use std::sync::Arc;
use crate::client::error::{FnsApiError, OpenApiClientError};
use crate::dto::ticket_request::GetTicketInfo;
use crate::dto::ticket_response::{Body, Envelope, GetMessageResponse};
use crate::traits::ticket::{TicketRequestErrorTrait, TicketResponseResult, TicketResponseTrait, TicketTrait};

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
            Body::GetMessageResponse(t) => {
                Ok(TicketResponse::ok(Ticket::new(t)))
            }
        }
    }
    fn ok(ticket: Arc<dyn TicketTrait>) -> Arc<dyn TicketResponseTrait> {
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
pub(super) struct Ticket;

impl TicketTrait for Ticket {}

impl Ticket {
    fn new(response: GetMessageResponse) -> Arc<dyn TicketTrait> {
        Arc::new(Ticket {})
    }
}