mod azdevops;
mod utils;

use azdevops::get_repo_list;

#[tokio::main]
async fn main() {
    let _repos = get_repo_list().expect("Failed to get repos");
}
