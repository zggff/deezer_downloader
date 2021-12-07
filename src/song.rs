use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Song {
    id: u64,
    title: String,
}

impl Song {
    pub async fn new(id: u64) -> Result<Song, Box<dyn Error>> {
        let url = format!("https://api.deezer.com/track/{}", id);
        let song = reqwest::get(url).await?.json::<Song>().await?;
        Ok(song)
    }

    // pub
}
