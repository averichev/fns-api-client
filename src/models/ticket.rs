use crate::dto::ticket_request::GetTicketInfo;
use crate::traits::ticket::{TicketRequestErrorTrait, TicketResponseResult, TicketResponseTrait, TicketTrait};

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
        TicketResponseResult::Err(TicketRequestError::new(String::from("какая-то ошибка")))
    }
}

struct TicketRequestError{
    message: String
}

impl TicketRequestError {
    fn new(message: String) -> Box<dyn TicketRequestErrorTrait> {
        Box::new(TicketRequestError{
            message
        })
    }
}

impl TicketRequestErrorTrait for TicketRequestError {
    fn message(&self) -> String {
        self.message.clone()
    }
}

impl TicketTrait for GetTicketInfo{
    fn sum(&self) -> f64 {
        self.sum
    }

    fn date(&self) -> String {
        self.date.clone()
    }

    fn r#fn(&self) -> u64 {
        self.r#fn
    }

    fn type_operation(&self) -> u8 {
        self.type_operation
    }

    fn fiscal_document_id(&self) -> u64 {
        self.fiscal_document_id
    }

    fn fiscal_sign(&self) -> u64 {
        self.fiscal_sign
    }
}