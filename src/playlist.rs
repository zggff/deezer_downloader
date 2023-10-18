use reqwest::Client;
use serde::Deserialize;

use crate::song::SongMetadata;

#[derive(Deserialize)]
pub struct Playlist {
    pub id: u64,
    pub title: String,
    #[serde(rename = "nb_tracks")]
    pub len: usize,
    pub tracks: Tracks,
}

impl Playlist {
    pub async fn get(id: u64, client: &Client) -> anyhow::Result<Self> {
        let url = format!("https://api.deezer.com/playlist/{id}");
        Ok(client.get(url).send().await?.json::<Playlist>().await?)
    }
}

#[derive(Deserialize)]
pub struct Tracks {
    pub data: Vec<SongMetadata>,
}
