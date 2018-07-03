extern crate github_rs;
extern crate serde_json;
use std::env;
use github_rs::client::{Executor, Github};
use serde_json::Value;

fn main() {
    let _gh_user = env::var("GITHUB_USER").unwrap();
    let gh_key = env::var("GITHUB_API_KEY").unwrap();
    let client = Github::new(gh_key).unwrap();
    let me = client.get()
                   .user()
                   .execute::<Value>();
    match me {
        Ok((headers, status, json)) => {
            println!("{}", headers);
            println!("{}", status);
            if let Some(json) = json{
                println!("{}", json);
            }
        },
        Err(e) => println!("{}", e)
    }
}