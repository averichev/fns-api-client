use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(
prefix = "soapenv",
namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/"
namespace = "ns: urn://x-artefacts-gnivc-ru/inplat/servin/OpenApiAsyncMessageConsumerService/types/1.0"
)]
pub(crate) struct Envelope {
    #[yaserde(rename = "Body")]
    pub(crate) body: Body,
}
#[derive(YaDeserialize, YaSerialize)]
#[yaserde(
prefix = "soapenv"
)]
pub(crate) struct Body {
    #[yaserde(rename = "GetMessagesRequest")]
    pub(crate) get_messages_request: GetMessagesRequest,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(
prefix = "ns"
)]
pub(crate) struct GetMessagesRequest {
    #[yaserde(rename = "Expressions")]
    pub(crate) expressions: Vec<Expressions>,
}
#[derive(YaDeserialize, YaSerialize)]
#[yaserde(
prefix = "ns"
)]
pub(crate) struct Expressions {
    #[yaserde(rename = "MessageId")]
    pub(crate) message_id: String,
    #[yaserde(rename = "UserToken")]
    pub(crate) user_token: String
}

