mod azdevops;
mod utils;

use azdevops::get_repo_list;

#[tokio::main]
async fn main() {
    let repos = get_repo_list().await.expect("Failed to get repos");
    for repo in repos {
        println!("{}", repo.name)
    }
}
