use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(
prefix = "soapenv",
namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/"
namespace = "ns: urn://x-artefacts-gnivc-ru/inplat/servin/OpenApiAsyncMessageConsumerService/types/1.0"
)]
pub(crate) struct Envelope {
    #[yaserde(rename = "Body", prefix = "soapenv")]
    pub(crate) body: Body,
}
#[derive(YaDeserialize, YaSerialize)]
pub(crate) struct Body {
    #[yaserde(rename = "GetMessagesRequest", prefix = "ns")]
    pub(crate) get_messages_request: GetMessagesRequest,
}

#[derive(YaDeserialize, YaSerialize)]
pub(crate) struct GetMessagesRequest {
    #[yaserde(rename = "Expressions", prefix = "ns")]
    pub(crate) expressions: Expressions,
}
#[derive(YaDeserialize, YaSerialize)]
pub(crate) struct Expressions {
    #[yaserde(rename = "MessageId", prefix = "ns")]
    pub(crate) message_id: String,
    #[yaserde(rename = "UserToken", prefix = "ns")]
    pub(crate) user_token: String
}

