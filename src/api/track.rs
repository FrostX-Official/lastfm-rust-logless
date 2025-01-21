mod add_tags;
mod get_correction;
mod get_info;
mod get_similar;
mod get_tags;
mod get_top_tags;
mod love;
mod remove_tag;
mod scrobble;
mod search;
mod unlove;
mod update_now_playing;

use crate::Lastfm;

pub use add_tags::TrackAddTags;
pub use get_correction::TrackGetCorrection;
pub use get_info::TrackGetInfo;
pub use get_similar::TrackGetSimilar;
pub use get_tags::TrackGetTags;
pub use get_top_tags::TrackGetTopTags;
pub use love::TrackLove;
pub use remove_tag::TrackRemoveTag;
pub use scrobble::TrackScrobble;
pub use search::TrackSearch;
pub use unlove::TrackUnlove;
pub use update_now_playing::TrackUpdateNowPlaying;

/// Represents tag-related operations in the Last.fm API.
#[derive(Debug)]
pub struct Track<'a> {
    lastfm: &'a Lastfm,
}

impl<'a> Track<'a> {
    /// Creates a new `Track` instance.
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        Self { lastfm }
    }

    /// Creates a request to get info for the track.
    pub fn get_info(&mut self) -> TrackGetInfo<'_> {
        TrackGetInfo::new(self.lastfm)
    }

    /// Creates a request to get similar tracks.
    pub fn get_similar(&mut self) -> TrackGetSimilar<'_> {
        TrackGetSimilar::new(self.lastfm)
    }

    /// Creates a request to get tags for the track.
    pub fn get_tags(&mut self) -> TrackGetTags<'_> {
        TrackGetTags::new(self.lastfm)
    }

    /// Creates a request to get top tags for the track.
    pub fn get_top_tags(&mut self) -> TrackGetTopTags<'_> {
        TrackGetTopTags::new(self.lastfm)
    }

    /// Creates a request to love the track.
    pub fn love(&mut self) -> TrackLove<'_> {
        TrackLove::new(self.lastfm)
    }

    /// Creates a request to unlove the track.
    pub fn unlove(&mut self) -> TrackUnlove<'_> {
        TrackUnlove::new(self.lastfm)
    }

    /// Creates a request to add tags to the track.
    pub fn add_tags(&mut self) -> TrackAddTags<'_> {
        TrackAddTags::new(self.lastfm)
    }

    /// Creates a request to remove a tag from the track.
    pub fn remove_tag(&mut self) -> TrackRemoveTag<'_> {
        TrackRemoveTag::new(self.lastfm)
    }

    /// Creates a request to scrobble the track.
    pub fn scrobble(&mut self) -> TrackScrobble<'_> {
        TrackScrobble::new(self.lastfm)
    }

    /// Creates a request to update now playing for the track.
    pub fn update_now_playing(&mut self) -> TrackUpdateNowPlaying<'_> {
        TrackUpdateNowPlaying::new(self.lastfm)
    }

    /// Creates a request to get a correction from the track.
    pub fn get_correction(&mut self) -> TrackGetCorrection<'_> {
        TrackGetCorrection::new(self.lastfm)
    }

    /// Creates a request to search for a track.
    pub fn search(&mut self) -> TrackSearch<'_> {
        TrackSearch::new(self.lastfm)
    }
}
