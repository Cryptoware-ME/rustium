// use axum::{
//     http::header::{self, HeaderValue},
//     http::StatusCode,
//     response::{IntoResponse, IntoResponseParts, Response, ResponseParts},
// };
// use bytes::{BufMut, BytesMut};
// use serde::Serialize;
// use tracing::error;

// use crate::errors::Error;

// pub type CustomResponseResult<T> = Result<CustomResponse<T>, Error>;

// #[derive(Debug)]
// pub struct CustomResponse<T: Serialize> {
//     pub data: Option<T>,
//     pub status_code: StatusCode,
//     pub pagination: Option<ResponsePagination>,
// }

// impl<T> Default for CustomResponse<T>
// where
//     T: Serialize,
// {
//     fn default() -> Self {
//         Self {
//             data: None,
//             status_code: StatusCode::ok,
//             pagination: None,
//         }
//     }
// }

// impl<T> CustomResponse<T>
// where
//     T: Serialize,
// {
//     pub fn new() -> Self {
//         Self::default()
//     }

//     pub fn body(mut self, body: T) -> Self {
//         self.body = Some(body);
//         self
//     }

//     pub fn status_code(mut self, status_code: StatusCode) -> Self {
//         self.status_code = status_code;
//         self
//     }

//     pub fn pagination(mut self, pagination: ResponsePagination) -> Self {
//         self.pagination = Some(pagination);
//         self
//     }

//     pub fn build(self) -> CustomResponse<T> {
//         CustomResponse {
//             body: self.body,
//             status_code: self.status_code,
//             pagination: self.pagination,
//         }
//     }
// }

pub enum ResponseEnum {}
