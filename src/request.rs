use reqwest::blocking::Client;
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;
use serde_json::json;
use std::collections::HashMap;

const URL: &str = "https://api.linear.app/graphql";

pub fn gql(
    token: String,
    query: String,
    variables: HashMap<String, String>,
) -> Result<String, String> {
    let authorization: &str = &format!("Bearer {token}");

    let body = json!({"query": query, "variables": variables});

    let response = Client::new()
        .post(URL)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, authorization)
        .json(&body)
        .send()
        .or(Err("Did not get response from server"))?;

    if response.status().is_success() {
        Ok(response.text().or(Err("Could not read response text"))?)
    } else {
        Err(format!("Error: {:#?}", response.text()))
    }
}
