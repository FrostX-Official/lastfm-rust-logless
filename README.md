# Lastfm-rust: Rust library for accessing the Last.fm API.

## Overview

`lastfm-rust` is a Rust library that provides user for interacting with the Last.fm API.

You can view Last.fm's official API documentation [here](https://www.last.fm/api/).

## Install

Run this command in your terminal to add the latest version of `lastfm-rust`.
```bash
cargo add lastfm-rust
```

## Usage

Before you begin, make sure you have the following environment variables set up for authentication:

- `API_KEY`
- `API_SECRET`
- `SK` (session key) `only if you already have one`

These keys are required for making API requests.

### Example Setup:

1. **Set Up Environment Variables:**

You can create a `.env` file in your project directory with the following content:

```env
API_KEY=your_api_key
API_SECRET=your_api_secret
SK=your_session_key
```

2. **Usage Example:**

Here is a simple example of how to use the `lastfm-rust` library to interact with the API:

```rust
use dotenv::dotenv;
use lastfm_rust::Lastfm;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY env variable is required");
    let api_secret = std::env::var("API_SECRET").expect("API_SECRET env variable is required");
    let sk = std::env::var("SK").expect("SK env variable is required");

    // Initialize Lastfm client
    let lastfm = Lastfm::builder()
        .api_key(api_key)
        .api_secret(api_secret)
        .session_key(sk)
        .build()?;

    // Perform an action (e.g., add tags to an album)
    let response = lastfm
        .album()
        .add_tags()
        .artist("artist_name")
        .album("album_name")
        .tags("tag_name")
        .send()
        .await?;

    println!("{:?}", response);

    Ok(())
}
```

### Example: Get Session Key

Here’s how you can get session key using the `auth()` method:

```rust
let lastfm = Lastfm::builder()
    .api_key(api_key)
    .api_secret(api_secret)
    .build()?;

// req token
let response = lastfm.auth().get_token().send().await?;
let token = response["token"].clone();

// authorize the token and get session_key
lastfm.auth().pls_authorize(token.to_string());
let response = lastfm
    .auth()
    .get_session()
    .token(&token.to_string())
    .send()
    .await?;

println!("{response}");
```


## Features

- [x] **Album**
- [x] **Auth**
- [x] **Artist**
- [x] **Chart**
- [x] **Geo**
- [x] **Library**
- [ ] **Tag**
- [ ] **Track**
- [ ] **User**


## Example Requests

<details>
    <summary>Get album info.</summary>

```rust
let album_info = lastfm
    .album()
    .get_info()
    .artist("artist_name")
    .album("album_name")
    .send()
    .await?;
println!("{:?}", album_info);
```
</details>

<details>
    <summary>Get tags for an album.</summary>

```rust
let tags = lastfm
    .album()
    .get_tags()
    .artist("artist_name")
    .album("album_name")
    .send()
    .await?;
println!("{:?}", tags);
```
</details>

<details>
    <summary>Remove tags from an album.</summary>

```rust
let remove_response = lastfm
    .album()
    .remove_tags()
    .artist("artist_name")
    .album("album_name")
    .tags("tag_name")
    .send()
    .await?;
println!("Tags removed: {:?}", remove_response);
```
</details>

More examples can be found in the [examples](examples) folder.

## Contributing

Contributions are welcome! For any bug reports/feature requests, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
