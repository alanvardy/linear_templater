use reqwest::blocking::Client;
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;
use serde_json::json;
use std::fs;
use std::io::Write;

const PATH: &str = "/home/vardy/dev/linear_templater/test.json";
const URL: &str = "https://api.linear.app/graphql";

fn main() {
    let token = std::env::var("LINEAR_TOKEN").expect("LINEAR_TOKEN environment variable not set");
    let path = PATH.to_string();
    let query = "
        query {
            viewer {
                name
                id
                teamMemberships {
                    nodes {
                        team {
                            name
                            id
                        }
                    }
                }
            }
        }"
    .to_string();
    let result = gql_request(token, query).unwrap();

    write_json_to_file(result, path).unwrap();
}

fn gql_request(token: String, query: String) -> Result<String, String> {
    let authorization: &str = &format!("Bearer {token}");

    let body = json!({"query": query, "variables": {}});

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

fn write_json_to_file(json: String, path: String) -> std::result::Result<String, String> {
    let json = serde_json::from_str::<serde_json::Value>(&json).unwrap();
    let string = serde_json::to_string_pretty(&json).unwrap();

    let mut file = fs::File::create(&path).or(Err("Could not create file"))?;
    file.write_all(string.as_bytes())
        .or(Err("Could not write to file"))?;

    println!("Response written to {path}");

    Ok(String::from("✓"))
}
