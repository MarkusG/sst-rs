pub mod db;
pub mod model;
pub mod subcommands;
pub mod parser;
pub mod plaid {
    pub mod entities;
    pub mod requests;
    pub mod auth;
}

use hyper::StatusCode;

#[derive(Debug)]
pub struct HttpError {
    pub status_code: StatusCode
}

impl std::error::Error for HttpError {}

impl std::fmt::Display for HttpError {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.status_code)
    }

}
