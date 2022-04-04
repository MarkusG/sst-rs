use std::error::Error;

use crate::model::Transaction;

use time::OffsetDateTime;

pub fn parse(contents: &str, schema: &str, account: &str) -> Result<Vec<Transaction>, Box<dyn Error>> {
    match schema {
        "citi" => parse_citi(contents, account),
        _ => panic!()
    }
}

pub fn parse_citi(contents: &str, account: &str) -> Result<Vec<Transaction>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_reader(contents.as_bytes());

    let mut result: Vec<Transaction> = Vec::new();
    for r in reader.records().skip(1) {
        let record = r?;
        let debit = record.get(3).unwrap().parse::<f64>().unwrap_or(0.0);
        let credit = record.get(4).unwrap().parse::<f64>().unwrap_or(0.0);
        result.push(Transaction {
            id: None,
            account:  account.to_string(),
            // only one of these is ever populated at a time, I think...
            amount: debit + credit,
            category: None,
            description: Some(record.get(2).unwrap().to_string()),
            timestamp: OffsetDateTime::now_local()?
        });
    }

    Ok(result)
}
