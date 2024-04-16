use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Envelope {
    #[serde(rename = "Body")]
    pub(crate) body: Body,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Body {
    #[serde(rename = "GetMessagesResponse")]
    pub(crate) get_messages_response: GetMessagesResponse,
}

#[derive(Debug, Deserialize)]
pub(crate) struct GetMessagesResponse {
    #[serde(rename = "Messages")]
    pub(crate) messages: Vec<Messages>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Messages {
    #[serde(rename = "MessageId")]
    message_id: String,
    #[serde(rename = "Result")]
    pub(crate) result: Option<Result>,
    #[serde(rename = "Fault")]
    pub(crate) fault: Option<Fault>,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Result {
    #[serde(rename = "ProcessingStatus")]
    pub(crate) processing_status: String,
    #[serde(rename = "Message")]
    pub(crate) message: Option<Message>,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Message {
    #[serde(rename = "GetTicketResponse")]
    pub(crate) get_ticket_response: GetTicketResponse,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct GetTicketResponse {
    #[serde(rename = "Result")]
    pub(crate) result: TicketResult,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct TicketResult {
    #[serde(rename = "Ticket")]
    pub(crate) ticket: String,
    #[serde(rename = "Code")]
    code: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct  Fault{
    message: String
}
