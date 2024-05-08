use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub enum Method {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "OPTIONS")]
    Options,
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "CONNECT")]
    Connect,
    #[serde(rename = "TRACE")]
    Trace,
}

impl From<Method> for http::Method {
    fn from(value: Method) -> Self {
        match value {
            Method::Get => http::Method::GET,
            Method::Post => http::Method::POST,
            Method::Put => http::Method::PUT,
            Method::Delete => http::Method::DELETE,
            Method::Patch => http::Method::PATCH,
            Method::Options => http::Method::OPTIONS,
            Method::Head => http::Method::HEAD,
            Method::Connect => http::Method::CONNECT,
            Method::Trace => http::Method::TRACE,
        }
    }
}
