use crate::utils::get_credential;
use azure_devops_rust_api::git;
use azure_devops_rust_api::git::models::GitRepository;
use azure_devops_rust_api::graph;
use azure_devops_rust_api::graph::models::GraphSubjectQuery;
use std::env;
use std::error::Error;
use std::string::String;

pub async fn get_repo_list() -> Result<Vec<GitRepository>, Box<dyn Error>> {
    // Get authentication credential
    let credential = get_credential();

    // Get ADO server configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");

    // Create a git client
    let git_client = git::ClientBuilder::new(credential).build();

    // Get all repositories in the specified organization/project
    let repos = git_client
        .repositories_client()
        .list(organization, project)
        .await?;

    Ok(repos.value)
}

pub enum WorkItemStatus {
    InProgress,
    OnHold,
}

pub async fn get_work_items(status: WorkItemStatus) {
    let credential = get_credential();

    // Get ADO server configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");

    let graph_client = graph::ClientBuilder::new(credential).build();
    
	let query = GraphSubjectQuery {
        query: String::from("Select [System.Id], [System.Title], [System.State] From WorkItems Where [System.WorkItemType] = 'Task'").unwrap(), 
        scope_descriptor: None,
        subject_kind: vec!["User".to_string()],
    };

    let query = GraphSubjectQuery {
        
    };
}
