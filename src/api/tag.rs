mod get_info;
mod get_similar;
mod get_top_albums;
mod get_top_artists;
mod get_top_tags;
mod get_top_tracks;
mod get_weekly_chart_list;

use crate::Lastfm;

pub use get_info::TagGetInfo;
pub use get_similar::TagGetSimilar;
pub use get_top_albums::TagGetTopAlbums;
pub use get_top_artists::TagGetTopArtists;
pub use get_top_tags::TagGetTopTags;
pub use get_top_tracks::TagGetTopTracks;
pub use get_weekly_chart_list::TagGetWeeklyChartList;

/// Represents tag-related operations in the Last.fm API.
#[derive(Debug)]
pub struct Tag<'a> {
    lastfm: &'a Lastfm,
}

impl<'a> Tag<'a> {
    /// Creates a new `Tag` instance.
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        Self { lastfm }
    }

    /// Creates a request to get info for the tag.
    pub fn get_info(&mut self) -> TagGetInfo<'_> {
        TagGetInfo::new(self.lastfm)
    }

    /// Creates a request to get similar tags.
    pub fn get_similar(&mut self) -> TagGetSimilar<'_> {
        TagGetSimilar::new(self.lastfm)
    }

    /// Creates a request to get top albums for the tag.
    pub fn get_top_albums(&mut self) -> TagGetTopAlbums<'_> {
        TagGetTopAlbums::new(self.lastfm)
    }

    /// Creates a request to get top artists for the tag.
    pub fn get_top_artists(&mut self) -> TagGetTopArtists<'_> {
        TagGetTopArtists::new(self.lastfm)
    }

    /// Creates a request to get top tags for the tag.
    pub fn get_top_tags(&mut self) -> TagGetTopTags<'_> {
        TagGetTopTags::new(self.lastfm)
    }

    /// Creates a request to get top tracks for the tag.
    pub fn get_top_tracks(&mut self) -> TagGetTopTracks<'_> {
        TagGetTopTracks::new(self.lastfm)
    }

    /// Creates a request to get the weekly chart list for the tag.
    pub fn get_weekly_chart_list(&mut self) -> TagGetWeeklyChartList<'_> {
        TagGetWeeklyChartList::new(self.lastfm)
    }
}
