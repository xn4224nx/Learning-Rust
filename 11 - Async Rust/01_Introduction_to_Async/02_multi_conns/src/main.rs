/*
 * Chapter 1. Introduction to Async
 * Multiple Connections Between Server & Client
 */

use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let body = response.text().await?;
        println!("{}", body);
    } else {
        println!("Invalid Response: {}", response.status());
    }

    return Ok(());
}
