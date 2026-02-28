/// Filter expression parsing (placeholder)
pub fn parse_expression(_expr: &str) -> Result<FilterExpression, FilterError> {
    // TODO: Implement custom filter expression parsing
    Err(FilterError::NotImplemented)
}

#[derive(Debug)]
pub struct FilterExpression {
    pub raw: String,
}

#[derive(Debug, thiserror::Error)]
pub enum FilterError {
    #[error("Filter expression parsing not yet implemented")]
    NotImplemented,
}
