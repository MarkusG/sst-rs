use std::error::Error;

use sst::db;
use sst::subcommands;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let transactions = sst::plaid::requests::sync_transactions(
        "",
        "",
        ""
        ).await?;
    return Ok(());
    let args: Vec<String> = std::env::args().collect();
    db::ensure_created()?;
    return match args[1].as_str() {
        "add" => subcommands::add_transaction(&args[1..]),
        "list" => subcommands::list_transactions(&args[1..]),
        "delete" => subcommands::delete_transaction(&args[1..]),
        "import" => subcommands::import_transactions(&args[1..]),
        _ => { println!("unknown subommand: {}", args[1]); Ok(()) }
    };
}
