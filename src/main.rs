use std::sync::Arc;

use clap::{Parser, Subcommand};
use deezer_downloader::Downloader;
use futures::future::join_all;

// TODO: download by playlist
#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// download song by id
    Song {
        /// song id
        id: u64,
        /// output file, defaults to <id>.mp3
        out: Option<String>,
    },
    /// download multiple songs
    Songs {
        /// song id
        ids: Vec<u64>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let cli = Cli::parse();
    match cli.command {
        Command::Song { id, out } => {
            let out = out.unwrap_or(format!("{id}.mp3"));
            let downloader = Downloader::new().await?;
            let song = downloader.download_song(id).await?;
            song.write_to_file(out)?;
        }
        Command::Songs { ids } => {
            let downloader = Arc::new(Downloader::new().await?);
            let mut tasks = Vec::with_capacity(ids.len());
            for id in ids {
                let downloader = downloader.clone();
                tasks.push(tokio::spawn(async move {
                    let out = format!("{id}.mp3");
                    let Ok(song) = downloader.download_song(id).await else {
                        log::error!("failed to download: {id}");
                        return;
                    };
                    if song.write_to_file(out).is_err() {
                        log::error!("failed to write to file: {id}.mp3")
                    };
                }));
            }
            join_all(tasks).await;
        }
    }

    Ok(())
}
