#[cfg(test)]
mod tests {
    use lastfm_rust::LASTFM_API_URL;

    #[test]
    fn test_lastfm_api_url() {
        assert_eq!(LASTFM_API_URL, "http://ws.audioscrobbler.com/2.0/");
    }
}
