use hyper_tls::HttpsConnector;
use hyper::{Client, Body, Method, Request};
use serde_json::json;
use serde::Deserialize;

use crate::HttpError;
use crate::plaid::{
    entities::Transaction,
    auth::Credentials
};

#[derive(Debug, Deserialize)]
pub struct SyncResponse {
    added: Vec<Transaction>,
    modified: Vec<Transaction>,
    removed: Vec<Transaction>,
    has_more: bool,
    next_cursor: String
}

pub async fn sync_transactions(credentials: Credentials)
    -> Result<SyncResponse, Box<dyn std::error::Error>> {
    let body = json!(
        {
            "client_id": credentials.client_id,
            "secret": credentials.client_secret,
            "access_token": credentials.token
        });

    let req = Request::builder()
        .method(Method::POST)
        .uri("https://sandbox.plaid.com/transactions/sync")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))?;

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let resp = client.request(req).await?;
    if resp.status().is_client_error() || resp.status().is_server_error() {
        return Err(Box::new(HttpError { status_code: resp.status() }))
    }
    let bytes = hyper::body::to_bytes(resp.into_body()).await?;
    let resp_string = String::from_utf8(bytes.to_vec())?;
    let resp: SyncResponse = serde_json::from_str(&resp_string)?;

    Ok(resp)
}
