use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(
prefix = "soap",
namespace = "soap: http://schemas.xmlsoap.org/soap/envelope/"
)]
pub struct Envelope {
    #[yaserde(prefix="soap", rename="Body", namespace="http://schemas.xmlsoap.org/soap/envelope/")]
    pub body: Body,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(
prefix = "soap",
namespace = "soap: http://schemas.xmlsoap.org/soap/envelope/"
)]
pub enum Body {
    #[yaserde(rename="Fault", prefix="soap", namespace="http://schemas.xmlsoap.org/soap/envelope/")]
    Fault(Fault),
    #[yaserde(rename="GetMessageResponse" prefix="soap")]
    GetMessageResponse(GetMessageResponse),
}

#[derive(YaDeserialize, YaSerialize)]
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

#[derive(YaDeserialize, YaSerialize)]
pub struct GetMessageResponse {
    #[yaserde(prefix="ns", rename="Message")]
    pub message: Message,
}

#[derive(YaDeserialize, YaSerialize)]
pub struct Message {
    #[yaserde(rename="AuthResponse", prefix="tns")]
    pub auth_response: AuthResponse,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(
namespace = "tns: urn://x-artefacts-gnivc-ru/ais3/kkt/AuthService/types/1.0"
)]
pub struct AuthResponse {
    #[yaserde(prefix="tns", rename="Result")]
    pub auth_app_info: Result,
}

#[derive(YaDeserialize, YaSerialize)]
#[yaserde(
namespace = "tns: urn://x-artefacts-gnivc-ru/ais3/kkt/AuthService/types/1.0"
)]
pub struct Result {
    #[yaserde(prefix="tns", rename="Token")]
    pub token: String,
    #[yaserde(prefix="tns", rename="ExpireTime")]
    pub expire_time: String,
}