mod get_top_artists;
mod get_top_tracks;

use crate::Lastfm;

pub use get_top_artists::GeoGetTopArtists;
pub use get_top_tracks::GeoGetTopTracks;

/// Represents geo-related operations in the Last.fm API.
#[derive(Debug)]
pub struct Geo<'a> {
    lastfm: &'a Lastfm,
}

impl<'a> Geo<'a> {
    /// Creates a new `Geo` instance.
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        Self { lastfm }
    }

    /// Get the most popular artists on Last.fm by country
    pub fn get_top_artists(&mut self) -> GeoGetTopArtists<'_> {
        GeoGetTopArtists::new(self.lastfm)
    }

    /// Get the most popular tracks on Last.fm by country
    pub fn get_top_tracks(&mut self) -> GeoGetTopTracks<'_> {
        GeoGetTopTracks::new(self.lastfm)
    }
}
