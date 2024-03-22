pub trait TicketResponseTrait{
    fn result(&self) -> TicketResponseResult;
}

pub enum TicketResponseResult{
    Ok(Box<dyn TicketTrait>),
    Err(Box<dyn TicketRequestErrorTrait>)
}

pub trait TicketTrait{
    fn sum(&self) -> f64;
    fn date(&self) -> String;
    fn r#fn(&self) -> u64;
    fn type_operation(&self) -> u8;
    fn fiscal_document_id(&self) -> u64;
    fn fiscal_sign(&self) -> u64;
}

pub trait TicketRequestErrorTrait{
    fn message(&self) -> String;
}