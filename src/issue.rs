use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::Path;
extern crate walkdir;

use walkdir::WalkDir;

use crate::request;

const ISSUE_CREATE_DOC: &str = "mutation (
                    $title: String!
                    $teamId: String!
                    $assigneeId: String
                    $description: String,
                    $parentId: String
                    $projectId: String
                ) {
                issueCreate(
                    input: {
                        title: $title
                        teamId: $teamId
                        assigneeId: $assigneeId
                        description: $description
                        parentId: $parentId
                        projectId: $projectId
                    }
                ) {
                    issue {
                        id
                        url
                    }
                }
                }
                ";

#[derive(Deserialize)]
struct ParentIssue {
    title: String,
    team_id: String,
    project_id: Option<String>,
    assignee_id: Option<String>,
    description: Option<String>,
    children: Vec<ChildIssue>,
}

#[derive(Deserialize)]
struct ChildIssue {
    title: String,
    team_id: Option<String>,
    assignee_id: Option<String>,
    description: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct IssueCreateResponse {
    data: Data,
}
#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
struct Data {
    issueCreate: IssueCreate,
}

#[derive(Deserialize, Serialize)]
struct IssueCreate {
    issue: Issue,
}

#[derive(Deserialize, Serialize)]
struct Issue {
    id: String,
    url: String,
}

/// We want to support a file path or a directory
pub fn create_issues_from_file_or_dir(token: String, path: String) -> Result<String, String> {
    if Path::is_dir(Path::new(&path)) {
        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(is_issue_toml)
        {
            create_issues(token.clone(), entry.path().to_str().unwrap().to_string())?;
        }
        Ok("Done".to_string())
    } else {
        create_issues(token, path)
    }
}

fn create_issues(token: String, path: String) -> Result<String, String> {
    let mut toml_string = String::new();

    fs::File::open(path.clone())
        .or(Err("Could not find file"))?
        .read_to_string(&mut toml_string)
        .or(Err("Could not read to string"))?;

    println!("Processing {path}");

    let ticket: ParentIssue = toml::from_str(&toml_string).unwrap();

    let mut variables = HashMap::new();
    variables.insert("title".to_string(), ticket.title.clone());
    let team_id = ticket.team_id;
    variables.insert("teamId".to_string(), team_id.clone());
    let assignee_id = ticket.assignee_id.unwrap_or_default();
    variables.insert("assigneeId".to_string(), assignee_id.clone());
    let project_id = ticket.project_id.unwrap_or_default();
    variables.insert("projectId".to_string(), project_id);
    let description = ticket.description.unwrap_or_default();
    variables.insert("description".to_string(), description);

    let response = request::gql(token.clone(), ISSUE_CREATE_DOC, variables)?;
    let Issue { id, url } = extract_id_from_response(response)?;

    println!("- [{}] {}", id, url);

    for child in ticket.children.iter() {
        let mut variables = HashMap::new();
        variables.insert("title".to_string(), child.title.clone());
        let child_team_id = child.team_id.clone().unwrap_or_else(|| team_id.clone());
        variables.insert("teamId".to_string(), child_team_id);
        variables.insert("parentId".to_string(), id.clone());
        let child_a_id = child
            .assignee_id
            .clone()
            .unwrap_or_else(|| assignee_id.clone());
        variables.insert("assigneeId".to_string(), child_a_id);
        let child_description = child.description.clone().unwrap_or_default();
        variables.insert("description".to_string(), child_description);
        let response = request::gql(token.clone(), ISSUE_CREATE_DOC, variables)?.to_string();
        let Issue { id, url } = extract_id_from_response(response)?;
        println!("  - [{}] {}", id, url);
    }
    Ok("Done".to_string())
}

/// Returns true if it is a TOML file that can be processed
fn is_issue_toml(entry: &walkdir::DirEntry) -> bool {
    entry.file_name().to_str().unwrap().ends_with(".toml")
        && !entry.file_name().to_str().unwrap().contains("Cargo.toml")
}

/// Get the id from an issue response, needed for parent issues and terminal output
fn extract_id_from_response(response: String) -> Result<Issue, String> {
    let data: Result<IssueCreateResponse, _> = serde_json::from_str(&response);

    match data {
        Ok(IssueCreateResponse {
            data: Data {
                issueCreate: IssueCreate { issue },
            },
        }) => Ok(issue),
        Err(err) => Err(format!("Could not parse response for issue: {err:?}")),
    }
}
