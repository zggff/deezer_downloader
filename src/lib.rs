//! deezer downloader provides basic functionality needed to download a song from [deezer](http://deezer.com)

pub mod downloader;
pub mod error;
pub mod song;

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::downloader::Downloader;

    #[tokio::test]
    async fn download_song_by_id() -> Result<(), Box<dyn Error>> {
        let mut downloader = Downloader::new().await?;
        let song = downloader.download_song(92719900).await?;
        song.write(format!("{}.mp3", song.metadata.id))?;
        // file.write(&song)?;

        Ok(())
    }
}
