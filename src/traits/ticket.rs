use std::sync::Arc;

pub trait TicketResponseTrait{
    fn result(&self) -> TicketResponseResult;
}

#[derive(Clone)]
pub enum TicketResponseResult{
    Ok(Arc<dyn TicketTrait>),
    Err(Arc<dyn TicketRequestErrorTrait>)
}

pub trait TicketTrait{

}

pub trait TicketRequestErrorTrait{
    fn message(&self) -> String;
}