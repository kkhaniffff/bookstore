use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BookPayload {
    pub title: String,
    pub author: String,
    pub publication_date: chrono::NaiveDate,
    pub stock_quantity: i32,
    pub price: i32,
}

#[derive(Debug, Deserialize, Default)]
pub struct BookFilterPayload {
    pub title: Option<String>,
    pub author: Option<String>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

impl BookFilterPayload {
    pub fn title(&self) -> String {
        self.title
            .as_deref()
            .map_or("%".into(), |t| format!("{t}%"))
    }

    pub fn author(&self) -> String {
        self.author
            .as_deref()
            .map_or("%".into(), |a| format!("{a}%"))
    }

    pub fn offset(&self) -> i64 {
        self.offset.unwrap_or(0)
    }

    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(100)
    }
}
