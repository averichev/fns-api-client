use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(YaDeserialize, YaSerialize, Debug)]
#[yaserde(
prefix = "soap",
namespace = "soap: http://schemas.xmlsoap.org/soap/envelope/"
)]
pub struct Envelope {
    #[yaserde(prefix="soap", rename="Body", namespace="http://schemas.xmlsoap.org/soap/envelope/")]
    pub body: Body,
}

#[derive(YaDeserialize, YaSerialize, Debug)]
#[yaserde(
prefix = "soap",
namespace = "soap: http://schemas.xmlsoap.org/soap/envelope/"
)]
pub enum Body {
    #[yaserde(rename="Fault", prefix="soap", namespace="http://schemas.xmlsoap.org/soap/envelope/")]
    Fault(Fault),
    #[yaserde(rename="SendMessageResponse")]
    SendMessageResponse(SendMessageResponse),
}

#[derive(YaDeserialize, YaSerialize, Debug)]
pub struct Fault {
    pub faultcode: String,
    pub faultstring: String
}

impl Default for Body {
    fn default() -> Self {
        Body::Fault(Fault{ faultcode: "".to_string(), faultstring: "".to_string() })
    }
}

impl Default for Fault {
    fn default() -> Self {
        Fault{ faultcode: "".to_string(), faultstring: "".to_string() }
    }
}

#[derive(YaDeserialize, YaSerialize, Debug)]
#[yaserde(
namespace = "urn://x-artefacts-gnivc-ru/inplat/servin/OpenApiAsyncMessageConsumerService/types/1.0"
)]
pub struct SendMessageResponse {
    #[yaserde(rename="MessageId")]
    pub message_id: String,
}