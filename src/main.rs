mod repos;
mod utils;

use crate::repos::get_repo_list;

#[tokio::main]
async fn main() {
    let _repos = get_repo_list().expect("Failed to get repos");
}
