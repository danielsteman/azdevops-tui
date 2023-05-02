use crate::utils::get_credential;
use azure_devops_rust_api::git;
use std::env;
use std::error::Error;

pub async fn get_repo_list() -> Result<Vec<git::models::GitRepository>, Box<dyn Error>> {
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
