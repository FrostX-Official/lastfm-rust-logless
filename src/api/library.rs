mod get_artists;

use crate::Lastfm;

pub use get_artists::LibraryGetArtists;

/// Represents library-related operations in the Last.fm API.
#[derive(Debug)]
pub struct Library<'a> {
    lastfm: &'a Lastfm,
}

impl<'a> Library<'a> {
    /// Creates a new `Library` instance.
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        Self { lastfm }
    }

    /// A paginated list of all the artists in a user's library, with play counts and tag counts.
    pub fn get_artists(&mut self) -> LibraryGetArtists<'_> {
        LibraryGetArtists::new(self.lastfm)
    }
}
