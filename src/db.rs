use sqlite::{State, Statement};
use time::{UtcOffset, OffsetDateTime};

use crate::model::Transaction;

const DATABASE_STRING: &str = "sst.db";

pub fn upsert_transaction(transaction: &Transaction) {
    let t = transaction;

    let connection = sqlite::open(DATABASE_STRING).unwrap();
    let mut statement: Statement;

    // if the transaction has an ID, we assume it exists already and try to
    // update
    if let Some(id) = t.id {
        statement = connection
            .prepare(format!(r#"UPDATE transactions SET
                     timestamp = :ts,
                     amount = :amt,
                     account = :acc,
                     category = :cat,
                     description = :desc
                     WHERE id = {}"#, id))
            .unwrap();
    }
    // else, insert it as a new transaction
    else
    {
        statement = connection
            .prepare(r#"INSERT INTO transactions
                 (timestamp, amount, account, category, description) VALUES
                 (:ts, :amt, :acc, :cat, :desc)"#)
            .unwrap();
    }

    // bind statement parameters
    statement.bind_by_name(":ts", t.timestamp.unix_timestamp()).unwrap();
    statement.bind_by_name(":amt", t.amount).unwrap();
    statement.bind_by_name(":acc", &*t.account).unwrap();
    statement.bind_by_name(":cat", &*t.category).unwrap();
    statement.bind_by_name(":desc", &*t.description).unwrap();

    // not sure if this loop is necessary, but the sqlite crate's documentation
    // isn't very clear
    while statement.next().unwrap() != State::Done {}
}

pub fn get_transaction(id: i64) -> Option<Transaction> {
    let connection = sqlite::open(DATABASE_STRING).unwrap();
    let mut statement = connection
        .prepare(format!(r#"SELECT * FROM transactions
                         WHERE id = {}
                         LIMIT 1"#, id))
        .unwrap();

    if let State::Row = statement.next().unwrap() {
        Some(Transaction {
            id: Some(statement.read::<i64>(0).unwrap()),
            timestamp:
                // parse the timestamp as UTC
                OffsetDateTime::from_unix_timestamp(
                statement.read::<i64>(1).unwrap()).unwrap()
                // convert to local time
                .to_offset(UtcOffset::current_local_offset().unwrap()),
            account: statement.read::<String>(2).unwrap(),
            amount: statement.read::<f64>(3).unwrap(),
            category: statement.read::<String>(4).unwrap(),
            description: statement.read::<String>(5).unwrap()
        })
    }
    else
    {
        None
    }
}

pub fn delete_transaction(id: i32) {
    let connection = sqlite::open(DATABASE_STRING).unwrap();
    let mut statement = connection
        .prepare(format!(r#"DELETE FROM transactions
                         WHERE id = {}"#, id))
                 .unwrap();

    while statement.next().unwrap() != State::Done {}
}