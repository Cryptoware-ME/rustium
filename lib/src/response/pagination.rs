#[derive(Debug)]
pub struct ResponsePagination {
    pub count: u64,
    pub offset: u64,
    pub limit: u32,
}
