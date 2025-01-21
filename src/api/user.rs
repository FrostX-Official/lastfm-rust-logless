mod get_friends;
mod get_info;
mod get_loved_track;
mod get_personal_tags;
mod get_recent_tracks;
mod get_top_albums;
mod get_top_artists;
mod get_top_tags;
mod get_top_tracks;
mod get_weekly_album_chart;
mod get_weekly_artist_chart;
mod get_weekly_chart_list;
mod get_weekly_track_chart;

use crate::Lastfm;

pub use get_friends::UserGetFriends;
pub use get_info::UserGetInfo;
pub use get_loved_track::UserGetLovedTracks;
pub use get_personal_tags::UserGetPersonalTags;
pub use get_recent_tracks::UserGetRecentTracks;
pub use get_top_albums::UserGetTopAlbums;
pub use get_top_artists::UserGetTopArtists;
pub use get_top_tags::UserGetTopTags;
pub use get_top_tracks::UserGetTopTracks;
pub use get_weekly_album_chart::UserGetWeeklyAlbumChart;
pub use get_weekly_artist_chart::UserGetWeeklyArtistChart;
pub use get_weekly_chart_list::UserGetWeeklyChartList;
pub use get_weekly_track_chart::UserGetWeeklyTrackChart;

/// Represents user-related operations in the Last.fm API.
#[derive(Debug)]
pub struct User<'a> {
    lastfm: &'a Lastfm,
}

impl<'a> User<'a> {
    /// Creates a new `User` instance.
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        Self { lastfm }
    }

    /// Creates a request to get information about the user.
    pub fn get_info(&mut self) -> UserGetInfo<'_> {
        UserGetInfo::new(self.lastfm)
    }

    /// Creates a request to get the user's friends.
    pub fn get_friends(&mut self) -> UserGetFriends<'_> {
        UserGetFriends::new(self.lastfm)
    }

    /// Creates a request to get the user's loved tracks.
    pub fn get_loved_tracks(&mut self) -> UserGetLovedTracks<'_> {
        UserGetLovedTracks::new(self.lastfm)
    }

    /// Creates a request to get personal tags for the user.
    pub fn get_personal_tags(&mut self) -> UserGetPersonalTags<'_> {
        UserGetPersonalTags::new(self.lastfm)
    }

    /// Creates a request to get the user's recent tracks.
    pub fn get_recent_tracks(&mut self) -> UserGetRecentTracks<'_> {
        UserGetRecentTracks::new(self.lastfm)
    }

    /// Creates a request to get the user's top albums.
    pub fn get_top_albums(&mut self) -> UserGetTopAlbums<'_> {
        UserGetTopAlbums::new(self.lastfm)
    }

    /// Creates a request to get the user's top artists.
    pub fn get_top_artists(&mut self) -> UserGetTopArtists<'_> {
        UserGetTopArtists::new(self.lastfm)
    }

    /// Creates a request to get the user's top tags.
    pub fn get_top_tags(&mut self) -> UserGetTopTags<'_> {
        UserGetTopTags::new(self.lastfm)
    }

    /// Creates a request to get the user's top tracks.
    pub fn get_top_tracks(&mut self) -> UserGetTopTracks<'_> {
        UserGetTopTracks::new(self.lastfm)
    }

    /// Creates a request to get the user's weekly album chart.
    pub fn get_weekly_album_chart(&mut self) -> UserGetWeeklyAlbumChart<'_> {
        UserGetWeeklyAlbumChart::new(self.lastfm)
    }

    /// Creates a request to get the user's weekly artist chart.
    pub fn get_weekly_artist_chart(&mut self) -> UserGetWeeklyArtistChart<'_> {
        UserGetWeeklyArtistChart::new(self.lastfm)
    }

    /// Creates a request to get the user's weekly chart list.
    pub fn get_weekly_chart_list(&mut self) -> UserGetWeeklyChartList<'_> {
        UserGetWeeklyChartList::new(self.lastfm)
    }

    /// Creates a request to get the user's weekly track chart.
    pub fn get_weekly_track_chart(&mut self) -> UserGetWeeklyTrackChart<'_> {
        UserGetWeeklyTrackChart::new(self.lastfm)
    }
}
