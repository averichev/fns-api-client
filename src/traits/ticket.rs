pub trait TicketResponseTrait{
    fn result(&self) -> TicketResponseResult;
}

pub enum TicketResponseResult{
    Ok(Box<dyn TicketTrait>),
    Err(Box<dyn TicketRequestErrorTrait>)
}

pub trait TicketTrait{}
pub trait TicketRequestErrorTrait{
    fn mesage() -> String;
}