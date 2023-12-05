use reqwest::Client;
use serde::ser::Serialize;
use tokio;

pub enum Task {
    FW,
    BW,
    OFF,
    Speed(f64),
    Addr(String),
}

#[derive(Debug, Clone)]
pub enum Message {
    Addr(String),
    FW,
    BW,
    Stop,
    Speed(f64),
}

#[tokio::main]
pub async fn post<T: Serialize>(addr: &String, form_a: &str, form_b: T) {
    let c = Client::new();
    let p = [(form_a, form_b)];
    c.post(addr).form(&p).send().await.unwrap();
}