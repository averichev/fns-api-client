use reqwest::Error;

#[derive(Clone)]
pub enum OpenApiClientError {
    DeserializationError(XmlDeserializationError),
    FnsApiError(ApiError),
    HttpClientError(HttpClientError),
    Error(String)
}

#[derive(Clone)]
pub struct XmlDeserializationError {
    pub message: String,
}
#[derive(Clone)]
pub struct ApiError {
    pub message: String,
}

#[derive(Clone)]
pub struct HttpClientError{
    pub message: String
}

impl HttpClientError {
    pub(crate) fn new(request_error: Error) -> HttpClientError {
        HttpClientError{ message: request_error.to_string() }
    }
}