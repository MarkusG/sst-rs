use std::error::Error;

use crate::model::Transaction;
use crate::db::upsert_transaction;

use getopts::Options;
use time::OffsetDateTime;

pub fn add_transaction(args: &[String]) -> Result<(), Box<dyn Error>> {
    // amount is an unnamed argument, and it comes first
    let amount = args[1].parse::<f64>()?;

    // parse named arguments
    let mut opts = Options::new();
    opts
        .optopt("c", "category", "transaction category", "CATEGORY")
        .optopt("d", "description", "transaction description", "DESCRIPTION")
        .optopt("a", "account", "transaction account", "ACCOUNT")
        .optopt("t", "timestamp", "transaction timestamp", "TIMESTAMP")
        .optopt("", "tags", "transaction tags", "TAG1,TAG2,TAG3,...");

    let matches = opts.parse(&args[2..])?;

    // require account
    let account = match matches.opt_str("a") {
        Some(a) => a,
        // TODO return an error here once we have errors sorted out better
        None => panic!()
    };
    
    // create transaction object
    let mut transaction = Transaction {
        id: None,
        amount,
        account,
        timestamp: OffsetDateTime::now_utc(),
        category: matches.opt_str("c"),
        description: matches.opt_str("d")
    };

    // if timestamp supplied, replace the default of now
    if let Some(s) = matches.opt_str("t") {
        let unix = s.parse::<i64>()?;
        transaction.timestamp = OffsetDateTime::from_unix_timestamp(unix)?;
    }

    // TODO tags

    // insert transaction
    upsert_transaction(&transaction)?;

    Ok(())
}
