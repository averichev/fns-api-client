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
    pub(crate) result: Result,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Result {
    #[serde(rename = "ProcessingStatus")]
    pub(crate) processing_status: String,
    #[serde(rename = "Message")]
    message: Option<Message>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Message {
    #[serde(rename = "GetTicketResponse")]
    get_ticket_response: GetTicketResponse,
}

#[derive(Debug, Deserialize)]
pub(crate) struct GetTicketResponse {
    #[serde(rename = "Result")]
    result: TicketResult,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TicketResult {
    #[serde(rename = "Ticket")]
    ticket: String,
    #[serde(rename = "Code")]
    code: String,
}
