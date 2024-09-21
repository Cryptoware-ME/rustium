use axum::{
    http::header::{self, HeaderValue},
    http::StatusCode,
    response::{IntoResponse, IntoResponseParts, Response, ResponseParts},
};
use bytes::{BufMut, BytesMut};
use serde::Serialize;
use tracing::error;

use crate::response::{pagination::ResponsePagination, RustiumResponse};

impl<T> IntoResponse for RustiumResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let body = match self.data {
            Some(data) => data,
            None => return (self.status_code).into_response(),
        };

        let mut bytes = BytesMut::new().writer();
        if let Err(err) = serde_json::to_writer(&mut bytes, &body) {
            error!("Error serializing response body as JSON: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }

        let bytes = bytes.into_inner().freeze();
        let headers = [(
            header::CONTENT_TYPE,
            HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
        )];

        match self.pagination {
            Some(pagination) => (self.status_code, pagination, headers, bytes).into_response(),
            None => (self.status_code, headers, bytes).into_response(),
        }
    }
}

impl IntoResponseParts for ResponsePagination {
    type Error = (StatusCode, String);

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        res.headers_mut()
            .insert("x-pagination-count", self.count.into());

        res.headers_mut()
            .insert("x-pagination-offset", self.offset.into());

        res.headers_mut()
            .insert("x-pagination-limit", self.limit.into());

        Ok(res)
    }
}
