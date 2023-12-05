use reqwest::Client;
use serde::ser::Serialize;
use tokio;
#[tokio::main]
pub async fn post<T: Serialize>(addr: &String, form_a: &str, form_b: T) {
    let c = Client::new();
    let p = [(form_a, form_b)];
    c.post(addr).form(&p).send().await;
}
