use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(
prefix = "soapenv",
namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/"
namespace = "ns: urn://x-artefacts-gnivc-ru/inplat/servin/OpenApiMessageConsumerService/types/1.0"
)]
pub struct Envelope {
    #[yaserde(prefix = "soapenv", rename="Body")]
    pub body: Body,
}

#[derive(YaDeserialize, YaSerialize)]
pub struct Body {
    #[yaserde(prefix = "ns", rename="GetMessageRequest")]
    pub get_message_request: GetMessageRequest,
}

#[derive(YaDeserialize, YaSerialize)]
pub struct GetMessageRequest {
    #[yaserde(prefix = "ns", rename="Message")]
    pub message: Message,
}

#[derive(YaDeserialize, YaSerialize)]
pub struct Message {
    #[yaserde(rename="AuthRequest", prefix="tns")]
    pub auth_request: AuthRequest,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(
namespace = "tns: urn://x-artefacts-gnivc-ru/ais3/kkt/AuthService/types/1.0"
)]
pub struct AuthRequest {
    #[yaserde(prefix="tns", rename="AuthAppInfo")]
    pub auth_app_info: AuthAppInfo,
}

#[derive(YaDeserialize, YaSerialize)]
pub struct AuthAppInfo {
    #[yaserde(prefix="tns", rename="MasterToken")]
    pub master_token: String,
}