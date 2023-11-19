use std::{
    fs,
    path::PathBuf,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use clap::{Parser, Subcommand};
use deezer_downloader::{song::Song, Downloader, Playlist, SongMetadata};
use futures::future::join_all;
use indicatif::ProgressBar;

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
        #[arg(short, long)]
        out: Option<String>,
    },
    /// download multiple songs
    Songs {
        /// song id
        ids: Vec<u64>,
        /// output directory
        #[arg(short, long, default_value_t = String::from("."))]
        out: String,
    },
    /// download playlist
    Playlist {
        /// playlist id
        id: u64,
        /// output directory
        #[arg(short, long)]
        out: Option<String>,
    },
}

fn filename_from_metadata(metadata: &SongMetadata) -> String {
    let title = &metadata.title;
    let artist = &metadata.artist.name;
    format!("{artist} - {title}.mp3")
        .chars()
        .map(|c| match c {
            '/' => '_',
            c => c,
        })
        .collect()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let cli = Cli::parse();
    match cli.command {
        Command::Song { id, out } => {
            let downloader = Downloader::new().await?;
            let song = Song::download(id, &downloader).await?;
            let path = out.unwrap_or(filename_from_metadata(&song.metadata));
            dbg!(&path);
            song.write_to_file(path)?;
        }
        Command::Songs { ids, out } => {
            let downloader = Arc::new(Downloader::new().await?);
            let mut tasks = Vec::with_capacity(ids.len());
            let path = Arc::new(PathBuf::from(out));
            for id in ids {
                let downloader = downloader.clone();
                let out = path.clone();
                tasks.push(tokio::spawn(async move {
                    let Ok(song) = Song::download(id, &downloader).await else {
                        log::error!("failed to download: {id}");
                        return;
                    };
                    let filename = out.join(filename_from_metadata(&song.metadata));
                    if song.write_to_file(filename).is_err() {
                        log::error!("failed to write to file: {id}.mp3")
                    };
                }));
            }
            join_all(tasks).await;
        }
        Command::Playlist { id, out } => {
            let downloader = Arc::new(Downloader::new().await?);
            let playlist = Playlist::get(id, downloader.client()).await?;
            let mut tasks = Vec::with_capacity(playlist.len);
            let out = match out {
                Some(out) => out,
                None => playlist.title.clone(),
            };
            fs::create_dir_all(&out)?;
            let path = Arc::new(PathBuf::from(out));
            let songs: Vec<String> = playlist
                .tracks
                .data
                .iter()
                .map(|song| filename_from_metadata(song))
                .collect();
            // INFO: this writes all song names into m3u8 file regardless of download status
            fs::write(
                path.join(format!("{}.m3u8", playlist.title)),
                songs.join("\n"),
            )?;
            println!("STARTED DOWNLOADING");
            let cnt = Arc::new(AtomicUsize::new(0));
            let pb = ProgressBar::new(playlist.len as u64);

            for song in playlist.tracks.data {
                let downloader = downloader.clone();
                let path = path.clone();
                let pb = pb.clone();
                let cnt = cnt.clone();
                tasks.push(tokio::spawn(async move {
                    let filename = filename_from_metadata(&song);
                    let path = path.join(&filename);
                    if path.exists() {
                        log::debug!("file already exists {}, skip downloading", filename);
                        pb.inc(1);
                        cnt.fetch_add(1, Ordering::SeqCst);
                        return;
                    }
                    let Ok(song) = Song::download_from_metadata(song, &downloader).await else {
                        log::error!("failed to download: {}", filename);
                        pb.inc(1);
                        return;
                    };
                    if song.write_to_file(&path).is_err() {
                        log::error!("failed to write to file: {}", filename);
                        pb.inc(1);
                        return;
                    };
                    log::debug!("downloaded: {}", filename);
                    pb.inc(1);
                    cnt.fetch_add(1, Ordering::SeqCst);
                }));
            }
            join_all(tasks).await;
            println!(
                "DOWNLOADED: {} OUT OF {}",
                cnt.load(Ordering::Relaxed),
                playlist.len
            );
            pb.finish_and_clear();
        }
    }

    Ok(())
}
