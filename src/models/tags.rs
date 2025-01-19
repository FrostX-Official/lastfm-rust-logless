use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TagsResponse {
    #[serde(rename = "tags")]
    pub tags: Tags,
}

#[derive(Debug, Deserialize)]
pub struct Tags {
    #[serde(rename = "tag")]
    pub tag: Option<Vec<Tag>>,

    #[serde(rename = "#text")]
    pub text: Option<String>,

    #[serde(rename = "@attr")]
    pub attr: Attr,
}

#[derive(Debug, Deserialize)]
pub struct Tag {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Attr {
    pub artist: String,
    pub album: String,
}
