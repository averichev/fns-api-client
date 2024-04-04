use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(
prefix = "soapenv",
namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/"
namespace = "ns: urn://x-artefacts-gnivc-ru/inplat/servin/OpenApiAsyncMessageConsumerService/types/1.0"
)]
pub struct Envelope {
    #[yaserde(rename = "Body")]
    pub body: Body,
}
#[derive(YaDeserialize, YaSerialize)]
#[yaserde(
prefix = "soapenv"
)]
struct Body {
    #[yaserde(rename = "GetMessagesRequest")]
    get_messages_request: GetMessagesRequest,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(
prefix = "ns"
)]
struct GetMessagesRequest {
    #[yaserde(rename = "Expressions")]
    expressions: Vec<Expressions>,
}
#[derive(YaDeserialize, YaSerialize)]
#[yaserde(
prefix = "ns"
)]
struct Expressions {
    #[yaserde(rename = "MessageId")]
    message_id: String,
    #[yaserde(rename = "UserToken")]
    user_token: String
}

