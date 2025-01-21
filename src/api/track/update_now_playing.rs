use crate::{
    api::{LastfmMethod, ParameterBuilder},
    APIResponse, Lastfm, Result,
};
use reqwest::Method;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct TrackUpdateNowPlaying<'a> {
    lastfm: &'a Lastfm,
    pub artist: Option<String>,
    pub track: Option<String>,
    pub album: Option<String>,
    pub track_number: Option<String>,
    pub context: Option<String>,
    pub mbid: Option<String>,
    pub duration: Option<String>,
    pub album_artist: Option<String>,
    method: LastfmMethod,
}

impl<'a> TrackUpdateNowPlaying<'a> {
    pub(crate) fn new(lastfm: &'a Lastfm) -> Self {
        TrackUpdateNowPlaying {
            lastfm,
            artist: None,
            track: None,
            album: None,
            track_number: None,
            context: None,
            mbid: None,
            duration: None,
            album_artist: None,
            method: LastfmMethod::TrackUpdateNowPlaying,
        }
    }
    pub fn artist(mut self, artist: &str) -> Self {
        self.artist = Some(artist.to_string());
        self
    }

    pub fn track(mut self, track: &str) -> Self {
        self.track = Some(track.to_string());
        self
    }

    pub fn album(mut self, album: &str) -> Self {
        self.album = Some(album.to_string());
        self
    }

    pub fn track_number(mut self, track_number: &str) -> Self {
        self.track_number = Some(track_number.to_string());
        self
    }

    pub fn context(mut self, context: &str) -> Self {
        self.context = Some(context.to_string());
        self
    }

    pub fn mbid(mut self, mbid: &str) -> Self {
        self.mbid = Some(mbid.to_string());
        self
    }

    pub fn duration(mut self, duration: &str) -> Self {
        self.duration = Some(duration.to_string());
        self
    }

    pub fn album_artist(mut self, album_artist: &str) -> Self {
        self.album_artist = Some(album_artist.to_string());
        self
    }

    fn validate(&self) -> Result<()> {
        Ok(())
    }

    pub async fn send(self) -> Result<APIResponse<Value>> {
        self.validate()?;

        let mut builder = ParameterBuilder::new();

        builder = builder
            .add_optional("artist", self.artist)
            .add_optional("track", self.track)
            .add_optional("album", self.album)
            .add_optional("trackNumber", self.track_number)
            .add_optional("context", self.context)
            .add_optional("mbid", self.mbid)
            .add_optional("duration", self.duration)
            .add_optional("albumArtist", self.album_artist);

        let mut params = builder.build();

        let response = self
            .lastfm
            .send_request(self.method, &mut params, Method::GET)
            .await?;

        Ok(response)
    }
}
