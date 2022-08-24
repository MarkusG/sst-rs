use std::path::Path;
use std::error::Error;

// use sst::db;
// use sst::subcommands;

use sst::plaid::auth::Credentials;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    match sst::plaid::requests::sync_transactions(
        Credentials::from_file(Path::new("credentials.txt"))?).await {
        Ok(response) => println!("{:?}", response),
        Err(e) => println!("{}", e)
    };
    Ok(())
    // let args: Vec<String> = std::env::args().collect();
    // db::ensure_created()?;
    // return match args[1].as_str() {
    //     "add" => subcommands::add_transaction(&args[1..]),
    //     "list" => subcommands::list_transactions(&args[1..]),
    //     "delete" => subcommands::delete_transaction(&args[1..]),
    //     "import" => subcommands::import_transactions(&args[1..]),
    //     _ => { println!("unknown subommand: {}", args[1]); Ok(()) }
    // };
}
