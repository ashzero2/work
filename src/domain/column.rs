#[derive(Debug, Clone)]
pub struct Column {
    pub id: i64,
    pub name: String,
    pub order_index: f64,
    pub color: Option<String>,
    pub wip_limit: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct NewColumn {
    pub name: String,
    pub color: Option<String>,
    pub wip_limit: Option<i64>,
}
