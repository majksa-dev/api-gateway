use pingora::{
    http::{Method, ResponseHeader, StatusCode},
    proxy::Session,
    Result,
};
use std::future::Future;

use super::headers::GetHeader;

pub trait CheckCors {
    fn check_cors(&self, allowed: &Vec<(&String, &Vec<String>)>) -> bool;
}

pub trait ApplyCors {
    fn apply_cors(&mut self, origin: &String, method: &Method, headers: &Vec<String>)
        -> Result<()>;
}

pub trait Preflight {
    fn preflight(&mut self, headers: &Vec<String>) -> impl Future<Output = Result<bool>> + Send;
}

impl ApplyCors for ResponseHeader {
    fn apply_cors(
        &mut self,
        origin: &String,
        method: &Method,
        headers: &Vec<String>,
    ) -> Result<()> {
        self.insert_header("Access-Control-Allow-Origin", origin)?;
        self.insert_header("Access-Control-Allow-Methods", method.to_string())?;
        self.insert_header(
            "Access-Control-Allow-Headers",
            format!("Authorization,Content-Type,X-Api-Token,X-Real-IP,X-RateLimit-Limit,X-RateLimit-Remaining,X-RateLimit-Reset{}", headers.join(",")),
        )?;
        Ok(())
    }
}

impl Preflight for Session {
    async fn preflight(&mut self, headers: &Vec<String>) -> Result<bool> {
        let method = &self.req_header().method;
        if method != "OPTIONS" {
            return Ok(false);
        }
        let mut response = ResponseHeader::build(StatusCode::OK, None)?;
        response.apply_cors(&String::from("*"), method, &headers)?;
        self.write_response_header(Box::new(response)).await?;
        Ok(true)
    }
}

impl CheckCors for Session {
    fn check_cors(&self, allowed: &Vec<(&String, &Vec<String>)>) -> bool {
        let (origin, token) = match (self.get_header("Origin"), self.get_header("X-Api-Token")) {
            (Ok(origin), Ok(token)) => (origin, token),
            _ => return false,
        };
        allowed
            .iter()
            .any(|auth| *auth.0 == *token && auth.1.contains(&origin))
    }
}
