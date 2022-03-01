use sqlite::{State, Statement};
use time::{UtcOffset, OffsetDateTime};
use crypto::digest::Digest;
use crypto::sha1::Sha1;

use crate::model::Transaction;

const DATABASE_STRING: &str = "sst.db";

pub fn ensure_created() -> Result<(), sqlite::Error> {
    let connection = sqlite::open(DATABASE_STRING)?;
    let mut statement = connection
        .prepare("SELECT * FROM sqlite_schema")?;
    if let State::Done = statement.next()? {
        connection.execute(r#"
            CREATE TABLE transactions (
                id INTEGER PRIMARY KEY,
                timestamp INTEGER,
                account TEXT,
                amount REAL,
                category TEXT,
                description TEXT,
                checksum BLOB UNIQUE
            );

            CREATE TABLE tags (
                id INTEGER PRIMARY KEY,
                transaction_id INTEGER,
                value TEXT,

                FOREIGN KEY(transaction_id) REFERENCES transactions(id)
            );

            CREATE TABLE notes (
                id INTEGER PRIMARY KEY,
                transaction_id INTEGER,
                value TEXT,

                FOREIGN KEY(transaction_id) REFERENCES transactions(id)
            );
        "#)?;
    }
    Ok(())
}

pub fn upsert_transaction(transaction: &Transaction) {
    let t = transaction;

    let connection = sqlite::open(DATABASE_STRING).unwrap();
    let mut statement: Statement;

    let mut hasher = Sha1::new();

    hasher.input(&t.timestamp.unix_timestamp().to_le_bytes());
    hasher.input(&t.amount.to_le_bytes());
    hasher.input_str(&t.account);
    hasher.input_str(&t.category);
    hasher.input_str(&t.description);
    let hash = hasher.result_str();

    // if the transaction has an ID, we assume it exists already and try to
    // update
    if let Some(id) = t.id {
        statement = connection
            .prepare(format!(r#"UPDATE transactions SET
                     timestamp = :ts,
                     amount = :amt,
                     account = :acc,
                     category = :cat,
                     description = :desc,
                     checksum = :checksum
                     WHERE id = {}"#, id))
            .unwrap();
    }
    // else, insert it as a new transaction
    else
    {
        statement = connection
            .prepare(r#"INSERT INTO transactions
                 (timestamp, amount, account, category, description, checksum)
                 VALUES (:ts, :amt, :acc, :cat, :desc, :checksum)"#)
            .unwrap();
    }

    // bind statement parameters
    statement.bind_by_name(":ts", t.timestamp.unix_timestamp()).unwrap();
    statement.bind_by_name(":amt", t.amount).unwrap();
    statement.bind_by_name(":acc", &*t.account).unwrap();
    statement.bind_by_name(":cat", &*t.category).unwrap();
    statement.bind_by_name(":desc", &*t.description).unwrap();
    statement.bind_by_name(":desc", &*t.description).unwrap();
    statement.bind_by_name(":checksum", &*hash).unwrap();

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
