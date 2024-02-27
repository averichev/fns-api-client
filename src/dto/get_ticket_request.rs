use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(
prefix = "soapenv",
namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/"
namespace = "ns: urn://x-artefacts-gnivc-ru/inplat/servin/OpenApiAsyncMessageConsumerService/types/1.0"
)]
pub struct Envelope {
    #[yaserde(prefix = "soapenv", rename="Body")]
    pub body: Body,
}

#[derive(YaDeserialize, YaSerialize)]
pub struct Body {
    #[yaserde(prefix = "ns", rename="SendMessageRequest")]
    pub send_message_request: SendMessageRequest,
}

#[derive(YaDeserialize, YaSerialize)]
pub struct SendMessageRequest {
    #[yaserde(prefix = "ns", rename="Message")]
    pub message: Message,
}

#[derive(YaDeserialize, YaSerialize)]
pub struct Message {
    #[yaserde(rename="GetTicketRequest", prefix="tns")]
    pub get_ticket_request: GetTicketRequest,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(
namespace = "tns: urn://x-artefacts-gnivc-ru/ais3/kkt/KktTicketService/types/1.0"
)]
pub struct GetTicketRequest {
    #[yaserde(prefix="tns", rename="GetTicketInfo")]
    pub get_ticket_info: GetTicketInfo,
}

#[derive(YaDeserialize, YaSerialize)]
pub struct GetTicketInfo {
    #[yaserde(prefix="tns", rename="Sum")]
    pub sum: f64,
    #[yaserde(prefix="tns", rename="Date")]
    pub date: String,
    #[yaserde(prefix="tns", rename="Fn")]
    pub r#fn: u64,
    #[yaserde(prefix="tns", rename="TypeOperation")]
    pub type_operation: u8,
    #[yaserde(prefix="tns", rename="FiscalDocumentId")]
    pub fiscal_document_id: u64,
    #[yaserde(prefix="tns", rename="FiscalSign")]
    pub fiscal_sign: u64,
}