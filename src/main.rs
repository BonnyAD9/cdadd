use std::{fmt::Display, path::Path};

use album_info::AlbumInfo;
use err::Result;
use flexi_logger::Logger;
use track_info::TrackInfo;

mod track_info;
mod err;
mod album_info;

fn main() -> Result<()> {
    Logger::try_with_env().unwrap().start()?;
    let album = AlbumInfo::from_dir("/home/kubas/test/test1")?;
    print_album(&album);
    Ok(())
}

fn print_album(album: &AlbumInfo) {
    println!("Album  : {}", field_str(album.album_title.as_ref()));
    println!("Artist : {}", field_str(album.album_performer.as_ref()));
    println!("Disc   : {}", field_str(album.disc.as_ref()));
    println!("CDINDEX: {}", field_str(album.cdindex_discid.as_ref()));
    println!("CDDB   : {}", field_str(album.cddb_discid.as_ref()));

    for (s, f) in album.tracks.iter() {
        println!();
        print_track(s, f);
    }
}

fn print_track(song: &TrackInfo, file: &Path) {
    println!("File        : {}", file.to_string_lossy());
    println!("Title       : {}", field_str(song.track_title.as_ref()));
    println!("Track       : {}", field_str(song.track_number.as_ref()));
    println!("Artist      : {}", field_str(song.performer.as_ref()));
    println!("ISRC        : {}", field_str(song.isrc.as_ref()));
    println!("Album       : {}", field_str(song.album_title.as_ref()));
    println!("Album artist: {}", field_str(song.album_performer.as_ref()));
    println!("Disc        : {}", field_str(song.disc.as_ref()));
    println!("CDINDEX     : {}", field_str(song.cdindex_discid.as_ref()));
    println!("CDDB        : {}", field_str(song.cddb_discid.as_ref()));
}

fn field_str<T>(field: Option<T>) -> String where T: Display {
    field.map_or_else(|| "--".to_owned(), |f| format!("{}", f))
}
