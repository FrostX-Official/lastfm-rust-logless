use dotenv::dotenv;
use lastfm_rust::{APIResponse, Lastfm};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY env variable is required");
    let api_secret = std::env::var("API_SECRET").expect("API_SECRET env variable is required");

    let lastfm = Lastfm::builder()
        .api_key(api_key)
        .api_secret(api_secret)
        .build()?;

    let response = lastfm.auth().get_token().send().await?;

    let token = match response {
        APIResponse::Success(value) => value.token,
        APIResponse::Error(err) => {
            format!("Error: {} - {}", err.error, err.message)
        }
    };
    println!("Token: {}", token);

    Ok(())
}
