use std::sync::Arc;

pub trait TicketResponseTrait{
    fn result(&self) -> TicketResponseResult;
}

#[derive(Clone)]
pub enum TicketResponseResult{
    Ok(Arc<dyn MessageTrait>),
    Err(Arc<dyn TicketRequestErrorTrait>)
}

pub trait MessageTrait {
    fn id(&self) -> String;
}

pub trait TicketRequestErrorTrait{
    fn message(&self) -> String;
}