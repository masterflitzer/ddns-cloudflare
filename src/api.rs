use reqwest::{header, Client as HttpClient, Response, Url};
use serde::Serialize;

pub(crate) async fn api_get(
    http: &HttpClient,
    url: Url,
    api_token: &str,
) -> Result<Response, reqwest::Error> {
    let response = http
        .get(url)
        .bearer_auth(api_token)
        .header(header::ACCEPT, "application/json")
        .send()
        .await?;
    Ok(response)
}

pub(crate) async fn api_patch<T: Serialize>(
    http: &HttpClient,
    url: Url,
    api_token: &str,
    body: T,
) -> Result<Response, reqwest::Error> {
    let response = http
        .patch(url)
        .bearer_auth(api_token)
        .header(header::ACCEPT, "application/json")
        .json(&body)
        .send()
        .await?;
    Ok(response)
}
