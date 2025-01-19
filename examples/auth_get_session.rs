use dotenv::dotenv;
use lastfm_rust::Lastfm;
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

    // req token
    let response = lastfm.auth().get_token().send().await?;

    let token = response["token"].clone();

    // authorize the token.
    lastfm.auth().pls_authorize(token.to_string());

    // get session_key
    let response = lastfm
        .auth()
        .get_session()
        .token(&token.to_string())
        .send()
        .await?;

    println!("Value: {response}");

    Ok(())
}
