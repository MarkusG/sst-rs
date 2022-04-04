use std::error::Error;

use sqlite::{State, Statement};
use time::OffsetDateTime;

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
                timestamp INTEGER NOT NULL,
                account TEXT NOT NULL,
                amount REAL NOT NULL,
                category TEXT,
                description TEXT
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

pub fn upsert_transaction(transaction: &Transaction) -> Result<(), Box<dyn Error>> {
    let t = transaction;

    let connection = sqlite::open(DATABASE_STRING)?;
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
                     WHERE id = {}"#, id))?;
    }
    // else, insert it as a new transaction
    else
    {
        statement = connection
            .prepare(r#"INSERT INTO transactions
                 (timestamp, amount, account, category, description)
                 VALUES (:ts, :amt, :acc, :cat, :desc)"#)?;
    }

    // bind statement parameters
    statement.bind_by_name(":ts", t.timestamp.unix_timestamp())?;
    statement.bind_by_name(":amt", t.amount)?;
    statement.bind_by_name(":acc", &*t.account)?;
    if let Some(c) = &t.category {
        statement.bind_by_name(":cat", &**c)?;
    }
    if let Some(d) = &t.description {
        statement.bind_by_name(":desc", &**d)?;
    }

    // not sure if this loop is necessary, but the sqlite crate's documentation
    // isn't very clear
    while statement.next()? != State::Done {}

    Ok(())
}

pub fn get_transaction(id: i64) -> Result<Option<Transaction>, Box<dyn Error>> {
    let connection = sqlite::open(DATABASE_STRING)?;
    let mut statement = connection
        .prepare(format!(r#"SELECT * FROM transactions
                         WHERE id = {}
                         LIMIT 1"#, id))?;
    if let State::Row = statement.next()? {
        Ok(Some(Transaction {
            id: Some(statement.read::<i64>(0)?),
            timestamp:
                // parse the timestamp as UTC
                OffsetDateTime::from_unix_timestamp(
                statement.read::<i64>(1)?)?,
            account: statement.read::<String>(2)?,
            amount: statement.read::<f64>(3)?,
            category: match statement.read::<String>(4) {
                Ok(s) => Some(s),
                Err(_) => None
            },
            description: match statement.read::<String>(5) {
                Ok(s) => Some(s),
                Err(_) => None
            }
        }))
    }
    else
    {
        Ok(None)
    }
}

pub fn list_transactions(count: Option<i32>) -> Result<Vec<Transaction>, Box<dyn Error>> {
    let connection = sqlite::open(DATABASE_STRING)?;

    // order by desc, to get most recent first
    let mut statement = match count {
        Some(c) => if c < 0 {
            // if count negative, get the c earliest transactions
                connection
                    .prepare(format!(r#"SELECT * FROM transactions
                         ORDER BY timestamp
                         LIMIT {}"#, -c))?
            }
            else
            {
            // else, get the c latest transactions
                connection
                    .prepare(format!(r#"SELECT * FROM transactions
                             ORDER BY timestamp DESC
                             LIMIT {}"#, c))?
            },
            // if count not provided, get all transactions, latest first
        None => connection
            .prepare(r#"SELECT * FROM transactions
                     ORDER BY timestamp DESC"#)?
    };


    let mut results: Vec<Transaction> = Vec::new();

    // populate results
    while let State::Row = statement.next()? {
        results.push(
            Transaction {
                id: Some(statement.read::<i64>(0)?),
                timestamp:
                    // parse the timestamp as UTC
                    OffsetDateTime::from_unix_timestamp(
                        statement.read::<i64>(1)?)?,
                    account: statement.read::<String>(2)?,
                    amount: statement.read::<f64>(3)?,
                    category: match statement.read::<String>(4) {
                        Ok(s) => Some(s),
                        Err(_) => None
                    },
                    description: match statement.read::<String>(5) {
                        Ok(s) => Some(s),
                        Err(_) => None
                    }
            })
    }

    Ok(results)
}

pub fn delete_transaction(id: i32) -> Result<(), Box<dyn Error>> {
    let connection = sqlite::open(DATABASE_STRING)?;
    let mut statement = connection
        .prepare(format!(r#"DELETE FROM transactions
                         WHERE id = {}"#, id))?;

    while statement.next()? != State::Done {}
    Ok(())
}
