# Deezer Loader

provides basic functionality needed to download a song from deezer

```rust 
use std::{error::Error, fs::File, io::Write};
use deezer_downloader::Downloader;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let downloader = Downloader::new().await?;
    let song = downloader.download_song(92719900).await?;
    let mut file = File::create("song.mp3")?;
    file.write(&song)?;

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
