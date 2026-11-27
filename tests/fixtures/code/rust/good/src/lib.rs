use std::fmt;

const MAX_RETRIES: u32 = 3;

#[derive(Debug, PartialEq, Eq)]
pub struct Invoice {
    id: String,
    total: String,
    is_paid: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError {
    row: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "cannot parse row {:?}: the id or the total is empty. Add both values.", self.row)
    }
}

impl Invoice {
    pub fn total(&self) -> &str {
        &self.total
    }
}

pub fn parse_invoice(row: &str) -> Result<Invoice, ParseError> {
    let fields: Vec<&str> = row.split(',').collect();
    let id = fields.first().copied().unwrap_or_default();
    let total = fields.get(1).copied().unwrap_or_default();
    if id.is_empty() || total.is_empty() {
        return Err(ParseError { row: row.to_owned() });
    }
    let is_paid = fields.get(2) == Some(&"yes");
    Ok(Invoice { id: id.to_owned(), total: total.to_owned(), is_paid })
}

pub fn can_retry(attempt: u32) -> bool {
    attempt < MAX_RETRIES
}
