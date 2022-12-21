# Deezer Loader

provides basic functionality needed to download a song from deezer

```rust 
use deezer_downloader::Downloader;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let downloader = Downloader::new().await?;
    let song = downloader.download_song(92719900).await?;
    song.write_to_file("song.mp3")?;

    Ok(())
}

```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE))
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT))

at your option.
