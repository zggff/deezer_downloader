use id3::{frame::Picture, Tag, Timestamp};
use reqwest::Client;
use serde::Deserialize;
use std::{fs::File, io::Write, path::Path, str::FromStr};

use crate::Downloader;

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
    pub release_date: Option<String>,
}

impl SongMetadata {
    pub async fn get(id: u64, client: &Client) -> anyhow::Result<Self> {
        let url = format!("https://api.deezer.com/track/{id}");
        Ok(client.get(url).send().await?.json::<SongMetadata>().await?)
    }
}

/// struct representing song with metadata, id3 tag and content
pub struct Song {
    pub metadata: SongMetadata,
    pub tag: Tag,
    pub content: Vec<u8>,
}

impl Song {
    /// download song
    pub async fn download(id: u64, downloader: &Downloader) -> anyhow::Result<Song> {
        let metadata = SongMetadata::get(id, downloader.client()).await?;
        Song::download_from_metadata(metadata, downloader).await
    }

    pub async fn download_from_metadata(
        metadata: SongMetadata,
        downloader: &Downloader,
    ) -> anyhow::Result<Song> {
        log::debug!("started download {}",metadata.id);

        let raw_data = downloader.dowload_raw_song_data(metadata.id).await?;
        let cover = downloader
            .client()
            .get(&metadata.album.cover_big)
            .send()
            .await?
            .bytes()
            .await?
            .to_vec();

        log::debug!("finished download {}",metadata.id);
        Song::from_raw_data_and_metadata(raw_data, metadata, cover).await
    }

    /// create new song instance from raw_data, and download metadata
    pub async fn from_raw_data_and_metadata(
        raw_data: Vec<u8>,
        metadata: SongMetadata,
        cover: Vec<u8>,
    ) -> anyhow::Result<Song> {
        let mut tag = Tag::new();
        tag.set_title(&metadata.title);
        tag.set_artist(&metadata.artist.name);
        tag.set_album(&metadata.album.title);
        if let Some(release_date) = &metadata.release_date {
            tag.set_date_released(Timestamp::from_str(release_date)?);
            tag.set_date_recorded(Timestamp::from_str(release_date)?);
        }
        tag.add_picture(Picture {
            mime_type: "image/jpeg".to_string(),
            picture_type: id3::frame::PictureType::CoverFront,
            description: "front cover".to_string(),
            data: cover,
        });
        Ok(Song {
            metadata,
            tag,
            content: raw_data,
        })
    }

    /// write song to file with id3 metadata
    pub fn write_to_file(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let mut file = File::create(&path)?;
        file.write_all(&self.content)?;
        self.tag.write_to_path(path, id3::Version::Id3v24)?;
        Ok(())
    }

    pub fn write(&self, output: &mut impl Write) -> anyhow::Result<()> {
        output.write_all(&self.content)?;
        Ok(())
    }
}
