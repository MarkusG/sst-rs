use hyper_tls::HttpsConnector;
use hyper::{Client, Body, Method, Request};
use serde_json::json;
use serde::Deserialize;
use crate::plaid::entities::Transaction;

#[derive(Debug, Deserialize)]
pub struct SyncResponse {
    added: Vec<Transaction>,
    modified: Vec<Transaction>,
    removed: Vec<Transaction>,
    has_more: bool,
    next_cursor: String
}

pub async fn sync_transactions(client_id: &str, client_secret: &str, token: &str)
    -> Result<SyncResponse, Box<dyn std::error::Error>> {
    let body = json!(
        {
            "client_id": client_id,
            "secret": client_secret,
            "access_token": token
        });

    let req = Request::builder()
        .method(Method::POST)
        .uri("https://sandbox.plaid.com/transactions/sync")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))?;

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let resp = client.request(req).await?;
    let bytes = hyper::body::to_bytes(resp.into_body()).await?;
    let resp_string = String::from_utf8(bytes.to_vec())?;
    let resp: SyncResponse = serde_json::from_str(&resp_string)?;
    println!("{:?}", resp);

    Ok(resp)
}
