use std::fmt;

use time::OffsetDateTime;

#[derive(Debug)]
pub struct Transaction {
    pub id: Option<i64>,
    pub timestamp: OffsetDateTime,
    pub account: String,
    pub amount: f64,
    pub category: String,
    pub description: String
}

impl Transaction {
    pub fn new(
        id: Option<i64>,
        timestamp: OffsetDateTime,
        account: String,
        amount: f64,
        category: String,
        description: String) -> Transaction {
        Transaction {
            id,
            timestamp,
            account,
            amount,
            category,
            description
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}|{}|{}|{}|{}|{}",
               match self.id {
                    Some(id) => id,
                    None => 0
               },
               self.timestamp,
               self.account,
               self.amount,
               self.category,
               self.description)
    }
}
