mod get_top_artists;
mod get_top_tags;
mod get_top_tracks;

use crate::Lastfm;

pub use get_top_artists::ChartGetTopArtists;
pub use get_top_tags::ChartGetTopTags;
pub use get_top_tracks::ChartGetTopTracks;

/// Represents chart-related operations in the Last.fm API.
#[derive(Debug)]
pub struct Chart<'a> {
    lastfm: &'a Lastfm,
}

impl<'a> Chart<'a> {
    /// Creates a new `Chart` instance.
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        Self { lastfm }
    }

    /// Creates a request to get top artists for the artist.
    pub fn get_top_artists(&mut self) -> ChartGetTopArtists<'_> {
        ChartGetTopArtists::new(self.lastfm)
    }

    /// Creates a request to get top tags for the artist.
    pub fn get_top_tags(&mut self) -> ChartGetTopTags<'_> {
        ChartGetTopTags::new(self.lastfm)
    }

    /// Creates a request to get top tracks for the artist.
    pub fn get_top_tracks(&mut self) -> ChartGetTopTracks<'_> {
        ChartGetTopTracks::new(self.lastfm)
    }
}
