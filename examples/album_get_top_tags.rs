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

    let response = lastfm
        .album()
        .get_top_tags()
        .artist("Billie Eilish")
        .album("HIT ME HARD AND SOFT")
        .autocorrect(true)
        .send()
        .await?;

    println!("Value: {response}");

    Ok(())
}
