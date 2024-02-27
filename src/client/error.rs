pub enum OpenApiClientError {
    DeserializationError(XmlDeserializationError),
    ApiError(ApiError),
    HttpClientError(HttpClientError)
}

pub struct XmlDeserializationError {
    pub message: String,
}
pub struct ApiError {
    pub message: String,
}

pub struct HttpClientError{
    pub message: String
}