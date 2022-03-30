use std::error::Error;

use sst::db;
use sst::subcommands;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    db::ensure_created().unwrap();
    return match args[1].as_str() {
        "add" => subcommands::add_transaction(&args[1..]),
        _ => { println!("unknown subommand: {}", args[1]); Ok(()) }
    };
    // db::upsert_transaction(&Transaction::new(
    //     Some(1),
    //     OffsetDateTime::now_local().unwrap(),
    //     "Banking Plus".to_string(),
    //     9.99,
    //     "Entertainment".to_string(),
    //     "Idk a movie or something".to_string()));

    // println!("{}", db::get_transaction(1).unwrap());
}
