use crate::{
    api::{LastfmMethod, ParameterBuilder},
    APIResponse, Error, Lastfm, Result,
};
use reqwest::Method;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct TrackScrobble<'a> {
    lastfm: &'a Lastfm,
    pub artist: Option<String>,
    pub track: Option<String>,
    pub timestamp: Option<u64>,
    pub album: Option<String>,
    pub context: Option<String>,
    pub stream_id: Option<String>,
    pub chosen_by_user: Option<u8>,
    pub track_number: Option<u8>,
    pub mbid: Option<String>,
    pub album_artist: Option<String>,
    pub duration: Option<u64>,
    method: LastfmMethod,
}

impl<'a> TrackScrobble<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        TrackScrobble {
            lastfm,
            artist: None,
            track: None,
            timestamp: None,
            album: None,
            context: None,
            stream_id: None,
            chosen_by_user: None,
            track_number: None,
            mbid: None,
            album_artist: None,
            duration: None,
            method: LastfmMethod::TrackScrobble,
        }
    }

    pub fn artist(mut self, artist: Option<&str>) -> Self {
        self.artist = artist.map(|s| s.to_string());
        self
    }

    pub fn track(mut self, track: Option<&str>) -> Self {
        self.track = track.map(|s| s.to_string());
        self
    }

    pub fn timestamp(mut self, timestamp: Option<u64>) -> Self {
        self.timestamp = timestamp;
        self
    }

    pub fn album(mut self, album: Option<&str>) -> Self {
        self.album = album.map(|s| s.to_string());
        self
    }

    pub fn context(mut self, context: Option<&str>) -> Self {
        self.context = context.map(|s| s.to_string());
        self
    }

    pub fn stream_id(mut self, stream_id: Option<&str>) -> Self {
        self.stream_id = stream_id.map(|s| s.to_string());
        self
    }

    pub fn chosen_by_user(mut self, chosen_by_user: Option<u8>) -> Self {
        self.chosen_by_user = chosen_by_user;
        self
    }

    pub fn track_number(mut self, track_number: Option<u8>) -> Self {
        self.track_number = track_number;
        self
    }

    pub fn mbid(mut self, mbid: Option<&str>) -> Self {
        self.mbid = mbid.map(|s| s.to_string());
        self
    }

    pub fn album_artist(mut self, album_artist: Option<&str>) -> Self {
        self.album_artist = album_artist.map(|s| s.to_string());
        self
    }

    pub fn duration(mut self, duration: Option<u64>) -> Self {
        self.duration = duration;
        self
    }

    fn validate(&self) -> Result<()> {
        if self.artist.is_none() || self.track.is_none() || self.timestamp.is_none() {
            return Err(Error::Generic(
                "'artist', 'track', and 'timestamp' are required.".to_string(),
            ));
        }
        Ok(())
    }

    pub async fn send(self) -> Result<APIResponse<Value>> {
        self.validate()?;

        let mut builder = ParameterBuilder::new();

        builder = builder
            .add_optional("artist", self.artist)
            .add_optional("track", self.track)
            .add_optional("timestamp", self.timestamp.map(|b| b.to_string()))
            .add_optional("album", self.album)
            .add_optional("context", self.context)
            .add_optional("streamId", self.stream_id)
            .add_optional("chosenByUser", self.chosen_by_user.map(|b| b.to_string()))
            .add_optional("trackNumber", self.track_number.map(|b| b.to_string()))
            .add_optional("mbid", self.mbid)
            .add_optional("albumArtist", self.album_artist)
            .add_optional("duration", self.duration.map(|b| b.to_string()));

        let mut params = builder.build();

        let response = self
            .lastfm
            .send_request(self.method, &mut params, Method::POST)
            .await?;

        Ok(response)
    }
}
