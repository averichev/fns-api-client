use crate::traits::ticket::{TicketRequestErrorTrait, TicketResponseResult, TicketResponseTrait};

pub(crate) struct TicketResponse {

}

impl TicketResponse {
    pub(crate) fn new(xml_string: String) -> Box<dyn TicketResponseTrait> {
        println!("{}", xml_string);

        Box::new(TicketResponse {  })
    }
}

impl TicketResponseTrait for TicketResponse {
    fn result(&self) -> TicketResponseResult {
        TicketResponseResult::Err(TicketRequestError::new())
    }
}

struct TicketRequestError;

impl TicketRequestError {
    fn new() -> Box<dyn TicketRequestErrorTrait> {
        Box::new(TicketRequestError{})
    }
}

impl TicketRequestErrorTrait for TicketRequestError {

}