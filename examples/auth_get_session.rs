use dotenv::dotenv;
use lastfm_rust::{APIResponse, Lastfm};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY env variable is required");
    let api_secret = std::env::var("API_SECRET").expect("API_SECRET env variable is required");

    let lastfm = Lastfm::builder()
        .api_key(api_key)
        .api_secret(api_secret)
        .build()?;

    // Request token
    let response = lastfm.auth().get_token().send().await?;

    let token = match response {
        APIResponse::Success(value) => value.token,
        APIResponse::Error(err) => {
            return Err(Box::new(err) as Box<dyn Error>);
        }
    };

    println!("Token: {}", token);

    // Authorize the token
    lastfm.auth().pls_authorize(token.to_string());

    // Get session key
    let response = lastfm.auth().get_session().token(&token).send().await?;

    println!("Session: {}", response);

    Ok(())
}
