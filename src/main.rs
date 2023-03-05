use clap::{Arg, Command};
use colored::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};

mod request;

extern crate clap;

const APP: &str = "Linear Templater";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = "Alan Vardy <alan@vardy.cc>";
const ABOUT: &str = "Create Linear Tickets from TOML files";

struct Arguments<'a> {
    fetch_ids: Option<&'a str>,
    create_tickets: Option<&'a str>,
    token: String,
}

fn main() {
    let token = std::env::var("LINEAR_TOKEN").expect("LINEAR_TOKEN environment variable not set");

    let app = Command::new(APP)
        .version(VERSION)
        .author(AUTHOR)
        .about(ABOUT);
    let matches = app
        .arg(
            Arg::new("fetch_ids")
                .short('f')
                .long("fetch_ids")
                .required(false)
                .value_name("JSON FILE OUTPUT PATH")
                .help("Fetch ids for player and teams, and output to provided path as a JSON file"),
        )
        .arg(
            Arg::new("create_tickets")
                .short('c')
                .long("create_tickets")
                .required(false)
                .value_name("PATH TO TOML FILE")
                .help("Read a TOML file and create tickets from it"),
        )
        .get_matches();

    let arguments = Arguments {
        fetch_ids: matches.get_one::<String>("fetch_ids").map(|s| s.as_str()),
        create_tickets: matches
            .get_one::<String>("create_tickets")
            .map(|s| s.as_str()),
        token,
    };

    match dispatch(arguments) {
        Ok(text) => {
            println!("{text}");
            std::process::exit(0);
        }
        Err(e) => {
            println!("{}", e.red());
            std::process::exit(1);
        }
    }
}

fn dispatch(arguments: Arguments) -> Result<String, String> {
    match arguments {
        Arguments {
            fetch_ids: Some(path),
            create_tickets: None,
            token,
        } => fetch_ids(token, path.to_string()),
        Arguments {
            fetch_ids: None,
            create_tickets: Some(path),
            token,
        } => create_tickets(token, path.to_string()),
        Arguments {
            fetch_ids: None,
            create_tickets: None,
            token: _,
        } => Err(String::from(
            "Linear Templater cannot be run without parameters. To see available parameters use --help",
        )),
        _ => Err(String::from(
            "Invalid parameters. For more information try --help",
        )),
    }
}

fn fetch_ids(token: String, path: String) -> Result<String, String> {
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
    let result = request::gql(token, query, HashMap::new())?;

    write_json_to_file(result, path)
}

#[derive(Deserialize)]
struct ParentTicket {
    title: String,
    team_id: String,
    assignee_id: Option<String>,
    description: Option<String>,
    children: Vec<ChildTicket>,
}

#[derive(Deserialize)]
struct ChildTicket {
    title: String,
    team_id: String,
    assignee_id: Option<String>,
    description: Option<String>,
}

fn create_tickets(token: String, path: String) -> Result<String, String> {
    let mut toml_string = String::new();

    fs::File::open(path)
        .or(Err("Could not find file"))?
        .read_to_string(&mut toml_string)
        .or(Err("Could not read to string"))?;

    let ticket: ParentTicket = toml::from_str(&toml_string).unwrap();

    let title = ticket.title;
    let team_id = ticket.team_id;
    let assignee_id = ticket.assignee_id.unwrap_or_default();
    let description = ticket.description.unwrap_or_default();

    let query = format!(
        "
        mutation ($description: String) {{
    issueCreate(input: {{
        title: \"{title}\", 
        teamId: \"{team_id}\", 
        assigneeId: \"{assignee_id}\",
        description: $description,
    }}) {{
        issue {{
            id
        }}
    }}
}}"
    );

    let mut variables = HashMap::new();
    variables.insert("description".to_string(), description);

    request::gql(token, query, variables)
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
