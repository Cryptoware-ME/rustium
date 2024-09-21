pub mod into_axum;
pub mod pagination;

use axum::http::StatusCode;
use serde::Serialize;

use crate::response::pagination::ResponsePagination;

#[derive(Debug)]
pub struct RustiumResponse<T: Serialize> {
    pub data: Option<T>,
    pub status_code: StatusCode,
    pub pagination: Option<ResponsePagination>,
}

impl<T> Default for RustiumResponse<T>
where
    T: Serialize,
{
    fn default() -> Self {
        Self {
            data: None,
            status_code: StatusCode::OK,
            pagination: None,
        }
    }
}

impl<T> RustiumResponse<T>
where
    T: Serialize,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }

    pub fn pagination(mut self, pagination: ResponsePagination) -> Self {
        self.pagination = Some(pagination);
        self
    }

    pub fn build(self) -> RustiumResponse<T> {
        RustiumResponse {
            data: self.data,
            status_code: self.status_code,
            pagination: self.pagination,
        }
    }
}
