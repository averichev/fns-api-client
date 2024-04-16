use std::fmt::{Display, Formatter};
use reqwest::Error;
use serde::de::StdError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
pub enum OpenApiClientError {
    DeserializationError(XmlDeserializationError),
    FnsApiError(FnsApiError),
    HttpClientError(HttpClientError),
    Error(String)
}

#[derive(Clone, Debug, Serialize)]
pub struct XmlDeserializationError {
    pub brief: String,
    pub cause: String,
    pub xml_string: String,
}

impl StdError for XmlDeserializationError {
}

impl Display for XmlDeserializationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*serde_json::to_string(&self.clone()).unwrap())
    }
}

#[derive(Clone, Serialize)]
pub struct FnsApiError {
    pub message: String,
}

#[derive(Clone, Serialize)]
pub struct HttpClientError{
    pub message: String
}

impl HttpClientError {
    pub(crate) fn new(request_error: Error) -> HttpClientError {
        HttpClientError{ message: request_error.to_string() }
    }
}