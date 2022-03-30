use std::error::Error;

use crate::model::Transaction;

use getopts::Options;
use time::OffsetDateTime;

/// sst add <amount> -a <account> [options]
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
    crate::db::upsert_transaction(&transaction)?;

    Ok(())
}

/// sst list [num]
/// gets [num] last transactions, or all transactions if [num] not provided
pub fn list_transactions(args: &[String]) -> Result<(), Box<dyn Error>> {
    // count is an optional unnamed argument, and it comes first
    let mut count: Option<i32> = None;
    if let Some(arg) = args.get(1) {
        count = Some(arg.parse::<i32>()?);
    }

    // placeholder
    // let mut opts = Options::new();

    // let matches = opts.parse(&args[2..])?;

    let mut transactions = crate::db::list_transactions(count)?;

    // reverse so the newest transaction is written last
    transactions.reverse();

    for t in transactions {
        println!("{}", t);
    }

    Ok(())
}
