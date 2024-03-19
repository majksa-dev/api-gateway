use pingora::{
    http::{RequestHeader, ResponseHeader},
    proxy::Session,
    Error, ErrorType, Result,
};

pub trait GetHeader {
    fn get_header(&self, header: &str) -> Result<String>;
}

pub trait SetHeader {
    fn set_header(&mut self, header: &'static str, value: &String) -> Result<()>;
}

impl GetHeader for RequestHeader {
    fn get_header(&self, header: &str) -> Result<String> {
        Ok(self
            .headers
            .get(header)
            .ok_or(Error::new(ErrorType::ConnectProxyFailure))?
            .to_str()
            .map_err(|_| Error::new(ErrorType::ConnectProxyFailure))?
            .to_string())
    }
}

impl GetHeader for Session {
    fn get_header(&self, header: &str) -> Result<String> {
        self.req_header().get_header(header)
    }
}

impl SetHeader for ResponseHeader {
    fn set_header(&mut self, header: &'static str, value: &String) -> Result<()> {
        self.insert_header(header, value)?;
        Ok(())
    }
}
