use time::OffsetDateTime;

use sst::db;
use sst::model::Transaction;

fn main() {
    db::ensure_created().unwrap();
    db::upsert_transaction(&Transaction::new(
        Some(1),
        OffsetDateTime::now_local().unwrap(),
        "Banking Plus".to_string(),
        9.99,
        "Entertainment".to_string(),
        "Idk a movie or something".to_string()));

    println!("{}", db::get_transaction(1).unwrap());
}
