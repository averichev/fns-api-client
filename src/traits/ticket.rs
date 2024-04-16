use std::fmt::{Debug, Display};
use std::sync::Arc;

pub trait TicketTrait{}

pub(crate) struct Ticket;

impl TicketTrait for Ticket {

}

pub trait TicketResponseTrait: Send + Sync{
    fn result(&self) -> TicketResponseResult;
}

#[derive(Clone)]
pub enum TicketResponseResult{
    Ok(Arc<dyn MessageTrait>),
    Err(Arc<dyn TicketRequestErrorTrait>)
}

pub trait MessageTrait: Send + Sync + Debug{
    fn id(&self) -> String;
}

pub trait TicketRequestErrorTrait: Send + Sync{
    fn message(&self) -> String;
}