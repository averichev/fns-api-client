use yaserde::de::from_str;

use crate::client::error::XmlDeserializationError;

pub(crate) struct Serde;

impl Serde {
    pub(crate) fn from_xml<T: yaserde::YaDeserialize>(xml_string: &str) -> Result<T, XmlDeserializationError> {
        let resp = from_str::<T>(xml_string);
        match resp {
            Ok(e) => {
                Ok(e)
            }
            Err(error) => {
                Err(
                    XmlDeserializationError {
                        brief: "Ошибка десериализации".to_string(),
                        cause: error,
                        xml_string: xml_string.to_string(),
                    }
                )
            }
        }
    }
}