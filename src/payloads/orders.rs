use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct OrderItemPayload {
    pub book_id: Uuid,
    pub amount: i32,
}

#[derive(Debug, Deserialize, Default)]
pub struct PaginationPayload {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

impl PaginationPayload {
    pub fn offset(&self) -> i64 {
        self.offset.unwrap_or(0)
    }

    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(100)
    }
}
