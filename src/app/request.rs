use log::error;
use pingora::{
    http::{RequestHeader, ResponseHeader, StatusCode},
    proxy::Session,
    Result,
};

use super::context::CTX;

pub trait RequestHandler {
    async fn handle_request(&self, session: &mut Session, ctx: &mut CTX) -> Option<StatusCode>;

    async fn modify_request(
        &self,
        session: &mut Session,
        request: &mut RequestHeader,
        ctx: &mut CTX,
    ) -> Result<()>;

    async fn modify_response(
        &self,
        session: &mut Session,
        response: &mut ResponseHeader,
        ctx: &mut CTX,
    ) -> Result<()>;
}

pub trait ToStatusCode<T> {
    fn to_status_code(self) -> Result<T, StatusCode>;
}

impl<T> ToStatusCode<T> for redis::RedisResult<T> {
    fn to_status_code(self) -> Result<T, StatusCode> {
        self.map_err(|e| {
            error!("{}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
    }
}
