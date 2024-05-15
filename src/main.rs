use std::{
    borrow::Cow,
    env,
    fmt::Display,
    io::{self, IsTerminal, Write},
    path::Path,
    process::ExitCode,
};

use album_info::AlbumInfo;
use err::Result;
use flexi_logger::Logger;
use termal::printmcln;
use track_info::TrackInfo;

use crate::cli::{Action, Args};

mod album_info;
mod cddb_read;
mod cli;
mod date;
mod err;
mod flac;
mod get_perf;
mod track_info;

fn main() -> ExitCode {
    match start() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            println!("{e}");
            ExitCode::FAILURE
        }
    }
}

fn start() -> Result<()> {
    Logger::try_with_env().unwrap().start()?;

    let args: Vec<_> = env::args().collect();
    let args = Args::parse(args.iter().into())?;
    match args.action() {
        Action::Help => help(),
        Action::Encode(p) => encode(&args, p.as_ref())?,
    }
    Ok(())
}

fn help() {
    let v = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");
    let is_term = io::stdout().is_terminal();
    let sign: Cow<str> = if is_term {
        termal::gradient("BonnyAD9", (250, 50, 170), (180, 50, 240)).into()
    } else {
        "BonnyAD9".into()
    };

    printmcln!(
        is_term,
        "Welcome in help for {'g i}cdadd{'_} by {sign}{'_}
Version {v}

{'g}Usage:
  {'c}cdadd {'w}-h{'_}
    Shows this help.

  {'c}cdadd {'w}-e <path> {'gr}[output dir] [{'dg}flags{'gr}]{'_}
    Encodes album in the folder given by {'w}path{'_}. Use {'y}-o{'_} if the
    {'gr}output dir{'_} starts with {'bold}-{'_}.

{'g}Flags:
  {'y}-h  -?  --help{'_}
    Shows this help.

  {'y}-e  --encode {'w}<path>{'_}
    Encodes album in the folder given by {'w}path{'_}.

  {'y}-i  --interactive{'_}
    Enables interactive mode for metadata.

  {'y}-o  --output{'_}
    Sets the output directory. This is {'i}cwd{'_} by default."
    );
}

fn encode(args: &Args, src: &str) -> Result<()> {
    let mut album = AlbumInfo::from_dir(src)?;
    if args.interactive && !configure(&mut album)? {
        return Ok(());
    }
    album.normalize();
    println!("Encoding:");
    flac::encode(&album, args.output())?;
    Ok(())
}

fn print_album(album: &AlbumInfo) {
    println!("Album    : {}", field_str(album.album_title.as_ref()));
    println!("Disc name: {}", field_str(album.disc_name.as_ref()));
    println!("Artist   : {}", field_str(album.artist.as_ref()));
    println!("Disc     : {}", field_str(album.disc));
    println!("CDINDEX  : {}", field_str(album.cdindex.as_ref()));
    println!(
        "CDDB     : {}",
        album
            .cddb
            .map_or_else(|| "--".to_owned(), |f| format!("{:x}", f))
    );
    println!("Date     : {}", field_str(album.date));
    println!("Genre    : {}", field_str(album.genre.as_ref()));

    for (s, f) in album.tracks.iter() {
        println!();
        print_track(s, f);
    }
}

fn print_track(song: &TrackInfo, file: &Path) {
    println!("File        : {}", file.to_string_lossy());
    println!("Title       : {}", field_str(song.title.as_ref()));
    println!("Track       : {}", field_str(song.track));
    println!("Artist      : {}", field_str(song.artist.as_ref()));
    println!("ISRC        : {}", field_str(song.isrc.as_ref()));
    println!("Date        : {}", field_str(song.date));
    println!("Genre       : {}", field_str(song.genre.as_ref()));
    println!("Album       : {}", field_str(song.album.as_ref()));
    println!("Disc name   : {}", field_str(song.disc_name.as_ref()));
    println!("Album artist: {}", field_str(song.album_artist.as_ref()));
    let ats = song.feat.join(", ");
    if ats.is_empty() {
        println!("Featuring   : --",);
    } else {
        println!("Featuring   : {ats}");
    }
    println!("Disc        : {}", field_str(song.disc));
    println!("CDINDEX     : {}", field_str(song.cdindex.as_ref()));
    println!(
        "CDDB        : {}",
        song.cddb
            .map_or_else(|| "--".to_owned(), |f| format!("{:x}", f))
    );
}

fn field_str<T>(field: Option<T>) -> String
where
    T: Display,
{
    field.map_or_else(|| "--".to_owned(), |f| format!("{}", f))
}

fn configure(album: &mut AlbumInfo) -> Result<bool> {
    print_album(album);
    let mut cmd = String::new();

    loop {
        print!("> ");
        _ = io::stdout().flush();
        cmd.clear();
        io::stdin().read_line(&mut cmd)?;
        if let Some(cmd) = cmd.trim().strip_prefix(':') {
            let cmd = cmd.trim_start().to_lowercase();
            match cmd.as_str() {
                "done" | "d" => return Ok(true),
                "quit" | "q" | "cancel" => return Ok(false),
                _ => println!("Unknown command '{}'", cmd),
            }
            continue;
        }

        let Some((fld, mut value)) =
            cmd.split_once(|c| matches!(c, ':' | '='))
        else {
            println!("Missing value for field");
            continue;
        };

        let fld = fld.trim().to_ascii_lowercase();
        value = value.trim();

        match fld.as_ref() {
            "disc name" => {
                album.disc_name = Some(value.to_owned());
                for (t, _) in album.tracks.iter_mut() {
                    t.disc_name = Some(value.to_owned());
                }
            }
            "album" => {
                album.album_title = Some(value.to_owned());
                for (t, _) in album.tracks.iter_mut() {
                    t.album = Some(value.to_owned());
                }
            }
            "artist" | "album artist" => {
                album.artist = Some(value.to_owned());
                for (t, _) in album.tracks.iter_mut() {
                    t.album_artist = Some(value.to_owned());
                }
            }
            "disc" => {
                let disc = match value.parse() {
                    Ok(d) => d,
                    Err(e) => {
                        println!("Failed to parse disc number: {e}");
                        continue;
                    }
                };
                album.disc = Some(disc);
                for (t, _) in album.tracks.iter_mut() {
                    t.disc = Some(disc);
                }
            }
            "cdindex" | "cdindex discid" => {
                album.cdindex = Some(value.to_owned());
                for (t, _) in album.tracks.iter_mut() {
                    t.cdindex = Some(value.to_owned());
                }
            }
            "cddb" | "cddb discid" => {
                let cddb = match u32::from_str_radix(value, 16) {
                    Ok(d) => d,
                    Err(e) => {
                        println!("Failed to parse disc number: {e}");
                        continue;
                    }
                };
                album.cddb = Some(cddb);
                for (t, _) in album.tracks.iter_mut() {
                    t.cddb = Some(cddb);
                }
            }
            "date" | "year" => {
                let date = match value.parse() {
                    Ok(d) => d,
                    Err(e) => {
                        println!("Failed to parse disc number: {e}");
                        continue;
                    }
                };
                album.date = Some(date);
                for (t, _) in album.tracks.iter_mut() {
                    t.date = Some(date);
                }
            }
            "genre" => {
                album.genre = Some(value.to_owned());
                for (t, _) in album.tracks.iter_mut() {
                    t.genre = Some(value.to_owned());
                }
            }
            _ => {
                println!("Unknown album field '{fld}'");
            }
        }
        print_album(album);
    }
}
