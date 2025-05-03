use std::{fmt::Display, fs, path::Path, process::Command};

use filesan::{replace_escape, Mode};
use log::error;

use crate::{album_info::AlbumInfo, err::Result, track_info::TrackInfo};

pub fn encode<P>(album: &AlbumInfo, dst: P) -> Result<()>
where
    P: AsRef<Path>,
{
    fs::create_dir_all(&dst)?;
    let mut tasks = vec![];
    for (i, (t, p)) in album.tracks.iter().enumerate() {
        let mut cmd = Command::new("flac");
        cmd.arg(p);
        cmd.args(["--best", "-o"]);
        cmd.arg(dst.as_ref().join(match (t.track, &t.title) {
            (Some(n), Some(t)) => {
                replace_escape(&format!("{n:02}. {t}.flac"), '-', Mode::all())
            }
            (Some(n), None) => format!("{n:02}.flac"),
            (None, Some(t)) => replace_escape(
                &format!("{:02}. {t}.flac", i + 1),
                '-',
                Mode::all(),
            ),
            _ => format!("{:02}.flac", i + 1),
        }));
        add_metadata(&mut cmd, t);
        tasks.push(cmd.spawn());
    }

    for t in tasks {
        match t.and_then(|mut t| t.wait()) {
            Ok(o) => {
                if !o.success() {
                    if let Some(c) = o.code() {
                        error!("Failed to encode: {c}");
                    } else {
                        error!("Failed to encode");
                    }
                }
            }
            Err(e) => {
                error!("Failed to encode: {e}");
            }
        }
    }

    Ok(())
}

fn add_metadata(cmd: &mut Command, track: &TrackInfo) {
    fn add_meta<T>(cmd: &mut Command, name: &str, value: Option<T>)
    where
        T: Display,
    {
        if let Some(value) = value {
            cmd.args(["-T", &format!("{name}={value}")]);
        }
    }

    // Fully standard
    add_meta(cmd, "TITLE", track.title.as_ref());
    add_meta(cmd, "ARTIST", track.artist.as_ref());
    add_meta(cmd, "ALBUM", track.album.as_ref());
    add_meta(cmd, "DATE", track.date);
    add_meta(cmd, "TRACKNUMBER", track.track);
    add_meta(cmd, "GENRE", track.genre.as_ref());
    add_meta(cmd, "ISRC", track.isrc.as_ref());

    for a in &track.feat {
        add_meta(cmd, "ARTIST", Some(a));
    }

    // Standard extensions
    add_meta(cmd, "DISCNUMBER", track.disc);
    add_meta(cmd, "VOLUME", track.disc_name.as_ref());

    // Non standard
    add_meta(cmd, "ALBUMARTIST", track.album_artist.as_ref());
    add_meta(cmd, "CDINDEX", track.cdindex.as_ref());
    if let Some(value) = track.cddb {
        cmd.args(["-T", &format!("CDDB={value:x}")]);
    }
}
