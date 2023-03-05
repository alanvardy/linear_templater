use clap::{Arg, Command};
use colored::*;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

mod issue;
mod request;

extern crate clap;

const APP: &str = "Linear Templater";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = "Alan Vardy <alan@vardy.cc>";
const ABOUT: &str = "Create Linear Tickets from TOML files";
const FETCH_IDS_DOC: &str = "
        query {
            viewer {
                name
                id
                teamMemberships {
                    nodes {
                        team {
                            name
                            id
                            projects {
                                nodes {
                                    name
                                    id
                                }
                            }
                        }
                    }
                }
            }
        }";

struct Arguments<'a> {
    fetch_ids: Option<&'a str>,
    create_issues: Option<&'a str>,
}

fn main() {
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
            Arg::new("create_issues")
                .short('c')
                .long("create_issues")
                .required(false)
                .value_name("PATH TO TOML FILE OR DIRECTORY")
                .help("Read a TOML file and create a issues from it"),
        )
        .get_matches();

    let arguments = Arguments {
        fetch_ids: matches.get_one::<String>("fetch_ids").map(|s| s.as_str()),
        create_issues: matches
            .get_one::<String>("create_issues")
            .map(|s| s.as_str()),
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
    let token = std::env::var("LINEAR_TOKEN").expect("LINEAR_TOKEN environment variable not set");
    check_for_latest_version();
    match arguments {
        Arguments {
            fetch_ids: Some(path),
            create_issues: None,
        } => fetch_ids(token, path.to_string()),
        Arguments {
            fetch_ids: None,
            create_issues: Some(path),
        } => issue::create_issues_from_file_or_dir(token, path.to_string()),
        Arguments {
            fetch_ids: None,
            create_issues: None,
        } => Err(String::from(
            "Linear Templater cannot be run without parameters. To see available parameters use --help",
        )),
        _ => Err(String::from(
            "Invalid parameters. For more information try --help",
        )),
    }
}

fn fetch_ids(token: String, path: String) -> Result<String, String> {
    let result = request::gql(token, FETCH_IDS_DOC, HashMap::new())?;

    write_json_to_file(result, path)
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

fn check_for_latest_version() {
    match request::get_latest_version() {
        Ok(version) if version.as_str() != VERSION => {
            println!(
                "Latest {} version is {}, found {}.\nRun {} to update if you installed with Cargo",
                APP,
                version,
                VERSION,
                format!("cargo install {APP} --force").bright_cyan()
            );
        }
        Ok(_) => (),
        Err(err) => println!(
            "{}, {:?}",
            format!("Could not fetch {APP} version from Cargo.io").red(),
            err
        ),
    };
}
