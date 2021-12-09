use id3::{frame::Picture, Tag, Timestamp};
use reqwest::Client;
use serde::Deserialize;
use std::{error::Error, fs::File, io::Write, path::Path, str::FromStr};

#[derive(Deserialize)]
pub struct Artist {
    pub id: u64,
    pub name: String,
}
#[derive(Deserialize)]
pub struct Album {
    pub id: u64,
    pub title: String,
    pub cover_small: String,
    pub cover_medium: String,
    pub cover_big: String,
}

/// structure representing most useful fields of song metadata from deezer api
#[derive(Deserialize)]
pub struct SongMetadata {
    pub id: u64,
    pub title: String,
    pub artist: Artist,
    pub album: Album,
    pub release_date: String,
}

/// struct representing song with metadata, id3 tag and content
pub struct Song {
    pub metadata: SongMetadata,
    pub tag: Tag,
    pub content: Vec<u8>,
}

impl Song {
    /// create new song instance
    pub async fn new(id: u64, raw_data: Vec<u8>, client: &Client) -> Result<Song, Box<dyn Error>> {
        let url = format!("https://api.deezer.com/track/{}", id);
        let metadata = client.get(url).send().await?.json::<SongMetadata>().await?;
        let cover_front = client
            .get(&metadata.album.cover_big)
            .send()
            .await?
            .bytes()
            .await?
            .to_vec();

        let mut tag = Tag::new();
        tag.set_title(&metadata.title);
        tag.set_artist(&metadata.artist.name);
        tag.set_album(&metadata.album.title);
        tag.set_date_released(Timestamp::from_str(&metadata.release_date)?);
        tag.set_date_recorded(Timestamp::from_str(&metadata.release_date)?);
        tag.add_picture(Picture {
            mime_type: "image/jpeg".to_string(),
            picture_type: id3::frame::PictureType::CoverFront,
            description: "front cover".to_string(),
            data: cover_front,
        });
        Ok(Song {
            metadata,
            tag,
            content: raw_data,
        })
    }

    /// write song to file
    pub fn write_to_file(&self, path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(&path)?;
        file.write_all(&self.content)?;
        self.tag.write_to_path(path, id3::Version::Id3v24)?;
        Ok(())
    }
}
