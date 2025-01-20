mod add_tags;
mod get_correction;
mod get_info;
mod get_similar;
mod get_tags;
mod get_top_albums;
mod get_top_tags;
mod get_top_tracks;
mod remove_tag;
mod search;

use crate::Lastfm;

pub use add_tags::ArtistAddTagsRequest;
pub use get_correction::ArtistGetCorrection;
pub use get_info::ArtistGetInfo;
pub use get_similar::ArtistGetSimilar;
pub use get_tags::ArtistGetTags;
pub use get_top_albums::ArtistGetTopAlbums;
pub use get_top_tags::ArtistGetTopTags;
pub use get_top_tracks::ArtistGetTopTracks;
pub use remove_tag::ArtistRemoveTag;
pub use search::ArtistSearch;

/// Represents artist-related operations in the Last.fm API.
#[derive(Debug)]
pub struct Artist<'a> {
    lastfm: &'a Lastfm,
}

impl<'a> Artist<'a> {
    /// Creates a new `Artist` instance.
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        Self { lastfm }
    }

    /// Creates a request to get tags for the artist.
    pub fn get_tags(&mut self) -> ArtistGetTags<'_> {
        ArtistGetTags::new(self.lastfm)
    }

    /// Creates a request to get corrections for the artist.
    pub fn get_correction(&mut self) -> ArtistGetCorrection<'_> {
        ArtistGetCorrection::new(self.lastfm)
    }

    /// Creates a request to get info for the artist.
    pub fn get_info(&mut self) -> ArtistGetInfo<'_> {
        ArtistGetInfo::new(self.lastfm)
    }

    /// Creates a request to get similar artist to this artist.
    pub fn get_similar(&mut self) -> ArtistGetSimilar<'_> {
        ArtistGetSimilar::new(self.lastfm)
    }

    /// Creates a request to add tags to the artist.
    pub fn add_tags(&mut self) -> ArtistAddTagsRequest<'_> {
        ArtistAddTagsRequest::new(self.lastfm)
    }

    /// Creates a request to get top albums for the artist.
    pub fn get_top_albums(&mut self) -> ArtistGetTopAlbums<'_> {
        ArtistGetTopAlbums::new(self.lastfm)
    }

    /// Creates a request to get top tags for the artist.
    pub fn get_top_tags(&mut self) -> ArtistGetTopTags<'_> {
        ArtistGetTopTags::new(self.lastfm)
    }

    /// Creates a request to get top tracks for the artist.
    pub fn get_top_tracks(&mut self) -> ArtistGetTopTracks<'_> {
        ArtistGetTopTracks::new(self.lastfm)
    }

    /// Creates a request to remove a tag for the artist.
    pub fn remove_tag(&mut self) -> ArtistRemoveTag<'_> {
        ArtistRemoveTag::new(self.lastfm)
    }

    /// Creates a request to search a artist.
    pub fn search(&mut self) -> ArtistSearch<'_> {
        ArtistSearch::new(self.lastfm)
    }
}
