mod add_tags;
mod get_info;
mod get_tags;
mod get_top_tags;
mod remove_tags;
mod search;

use crate::Lastfm;

pub use add_tags::AlbumAddTagsRequest;
pub use get_info::AlbumGetInfo;
pub use get_tags::AlbumGetTags;
pub use get_top_tags::AlbumGetTopTags;
pub use remove_tags::AlbumRemoveTag;
pub use search::AlbumSearch;

/// Represents album-related operations in the Last.fm API.
#[derive(Debug)]
pub struct Album<'a> {
    lastfm: &'a Lastfm,
}

impl<'a> Album<'a> {
    /// Creates a new `Album` instance.
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        Self { lastfm }
    }

    /// Creates a request to get tags for the album.
    pub fn get_tags(&mut self) -> AlbumGetTags<'_> {
        AlbumGetTags::new(self.lastfm)
    }

    /// Creates a request to add tags to the album.
    pub fn add_tags(&mut self) -> AlbumAddTagsRequest<'_> {
        AlbumAddTagsRequest::new(self.lastfm)
    }

    pub fn get_info(&mut self) -> AlbumGetInfo<'_> {
        AlbumGetInfo::new(self.lastfm)
    }

    pub fn get_top_tags(&mut self) -> AlbumGetTopTags<'_> {
        AlbumGetTopTags::new(self.lastfm)
    }

    pub fn remove_tag(&mut self) -> AlbumRemoveTag<'_> {
        AlbumRemoveTag::new(self.lastfm)
    }

    pub fn search(&mut self) -> AlbumSearch<'_> {
        AlbumSearch::new(self.lastfm)
    }
}
