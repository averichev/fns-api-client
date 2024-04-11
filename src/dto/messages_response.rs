use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(
prefix = "soap",
namespace = "soap: http://schemas.xmlsoap.org/soap/envelope/"
)]
pub(crate) struct Envelope {
    #[yaserde(rename = "Body", prefix = "soap")]
    pub(crate) body: Body,
}
#[derive(YaDeserialize, YaSerialize)]
pub(crate) struct Body {
    #[yaserde(rename = "GetMessagesResponse")]
    pub(crate) get_messages_response: GetMessagesResponse,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(
namespace = "urn://x-artefacts-gnivc-ru/inplat/servin/OpenApiAsyncMessageConsumerService/types/1.0"
)]
pub(crate) struct GetMessagesResponse {
    #[yaserde(rename = "Messages")]
    pub(crate) messages: Vec<Message>,
}
#[derive(YaDeserialize, YaSerialize)]
#[yaserde(
namespace = "urn://x-artefacts-gnivc-ru/inplat/servin/OpenApiAsyncMessageConsumerService/types/1.0"
)]
pub(crate) struct Message {
    #[yaserde(rename = "MessageId")]
    pub(crate) message_id: String,
    #[yaserde(rename = "Result")]
    pub(crate) result: Result
}

#[derive(YaDeserialize, YaSerialize)]
pub(crate) struct Result{
    #[yaserde(rename = "Message")]
    pub(crate) result_message: ResultMessage
}

#[derive(YaDeserialize, YaSerialize)]
pub(crate) struct ResultMessage{
    #[yaserde(rename = "GetTicketResponse", namespace = "urn://x-artefacts-gnivc-ru/ais3/kkt/KktTicketService/types/1.0")]
    pub(crate) get_ticket_response: GetTicketResponse
}

#[derive(YaDeserialize, YaSerialize)]
pub(crate) struct GetTicketResponse{
    #[yaserde(rename = "Result")]
    pub(crate) result: GetTicketResponseResult
}
#[derive(YaDeserialize, YaSerialize)]
pub(crate) struct GetTicketResponseResult{
    #[yaserde(rename = "Ticket")]
    pub(crate) ticket: String
}