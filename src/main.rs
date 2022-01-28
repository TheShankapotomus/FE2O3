use serde::Deserialize;
use gitlab::Gitlab;
use gitlab::api::{self, projects, issues, Query};


fn main() {
    println!("Hello, world!");

    //read these values from a config? or from bash? 
    let client = Gitlab::new("gitlab.com", "private-token").unwrap();

    let endpoint = issues::ProjectIssues::builder().project('maitai').label('status::releasable');
}
