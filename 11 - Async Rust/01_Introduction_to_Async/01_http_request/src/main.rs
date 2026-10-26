/*
 * Chapter 1. Introduction to Async
 * Http Requests Using Tokio
 */

use reqwest::Error;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://jsonplaceholder.typicode.com/posts/1";

    /* Time how long it takes to get the webpage asyncronously. */
    let mut async_requests = Vec::new();
    let start_time = Instant::now();

    for _ in 0..10 {
        async_requests.push(reqwest::get(url));
    }
    for req_result in async_requests.drain(..) {
        req_result.await?;
    }
    let elapsed_time = start_time.elapsed();
    println!("Async Web Request took: {} ms", elapsed_time.as_millis());

    /* Make the same requests in parallel. */
    let start_time = Instant::now();
    let (_, _, _, _, _, _, _, _, _, _) = tokio::join!(
        reqwest::get(url),
        reqwest::get(url),
        reqwest::get(url),
        reqwest::get(url),
        reqwest::get(url),
        reqwest::get(url),
        reqwest::get(url),
        reqwest::get(url),
        reqwest::get(url),
        reqwest::get(url),
    );
    let elapsed_time = start_time.elapsed();
    println!("Sync Web Request took:  {} ms", elapsed_time.as_millis());

    return Ok(());
}
