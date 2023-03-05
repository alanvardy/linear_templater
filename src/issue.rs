use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Read;

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
}

pub fn create_issues(token: String, path: String) -> Result<String, String> {
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
    let id = extract_id_from_response(response)?;

    println!("Created issue [{}] {}", id, ticket.title);

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
        let id = extract_id_from_response(response)?;
        println!("Created child [{}] {}", id, child.title);
    }
    Ok("Done".to_string())
}

fn extract_id_from_response(response: String) -> Result<String, String> {
    let data: Result<IssueCreateResponse, _> = serde_json::from_str(&response);

    match data {
        Ok(IssueCreateResponse {
            data:
                Data {
                    issueCreate:
                        IssueCreate {
                            issue: Issue { id },
                        },
                },
        }) => Ok(id),
        Err(err) => Err(format!("Could not parse response for issue: {err:?}")),
    }
}
