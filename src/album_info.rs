use std::{fs::{self, File}, io::BufReader, path::{Path, PathBuf}};

use log::warn;
use result::OptionResultExt;

use crate::{cddb_read::read_cddb, date::Date, err::Result, track_info::TrackInfo};

#[derive(Default, Debug)]
pub struct AlbumInfo {
    pub cdindex: Option<String>,
    pub cddb: Option<u32>,
    pub artist: Option<String>,
    pub disc_name: Option<String>,
    pub album_title: Option<String>,
    pub disc: Option<usize>,
    pub date: Option<Date>,
    pub genre: Option<String>,

    pub tracks: Vec<(TrackInfo, PathBuf)>,
}

impl AlbumInfo {
    pub fn from_dir<P>(path: P) -> Result<Self> where P: AsRef<Path> {
        let mut res = Self::default();
        res.load_dir(path.as_ref())?;
        Ok(res)
    }

    pub fn normalize(&mut self) {
        for (t, _) in &mut self.tracks {
            t.normalize();
        }
    }

    fn load_dir(&mut self, path: &Path) -> Result<()> {
        for f in fs::read_dir(path)? {
            let f = f?;
            if let Some(ex) = f.path().extension() {
                if ex != "wav" {
                    continue;
                }
            } else {
                continue;
            }

            let mut path = f.path().to_owned();
            path.set_extension("inf");
            match TrackInfo::from_file(path) {
                Ok(i) => self.tracks.push((i, f.path().into())),
                Err(e) => {
                    warn!("Failed to read info file for {:?}: {e}", f.path());
                    self.tracks.push((TrackInfo::default(), f.path().into()));
                }
            }
        }

        self.tracks.sort_by_key(|t| t.0.track);

        let cddb_file = path.join("audio.cddb");
        if let Err(e) = self.read_cddb(&cddb_file) {
            warn!("Failed to read cddb file {cddb_file:?}: {e}");
        }

        self.cdindex = self.cdindex.take().or_else(|| self.tracks.iter().flat_map(|(t, _)| t.cdindex.clone()).next());
        self.cddb = self.cddb.or_else(|| self.tracks.iter().flat_map(|(t, _)| t.cddb).next());
        self.artist = self.artist.take().or_else(|| self.tracks.iter().flat_map(|(t, _)| t.album_artist.clone()).chain(self.tracks.iter().flat_map(|(t, _)| t.artist.clone())).next());
        self.disc_name = self.disc_name.take().or_else(|| self.tracks.iter().flat_map(|(t, _)| t.disc_name.clone()).next());
        self.album_title = self.album_title.take().or_else(|| self.tracks.iter().flat_map(|(t, _)| t.album.clone()).next()).or_else(|| self.disc_name.clone());
        self.disc = self.disc.or_else(|| self.tracks.iter().flat_map(|(t, _)| t.disc).next());
        self.date = self.date.or_else(|| self.tracks.iter().flat_map(|(t, _)| t.date).max());
        self.genre = self.genre.take().or_else(|| self.tracks.iter().flat_map(|(t, _)| t.genre.clone()).next());

        for (t, _) in self.tracks.iter_mut() {
            t.cdindex = t.cdindex.take().or_else(|| self.cdindex.clone());
            t.cddb = t.cddb.or(self.cddb);
            t.artist = t.artist.take().or_else(|| self.artist.clone());
            t.disc_name = t.disc_name.take().or_else(|| self.disc_name.clone());
            t.disc = t.disc.or(self.disc);
            t.date = t.date.or(self.date);
            t.genre = t.genre.take().or_else(|| self.genre.clone());
        }

        Ok(())
    }

    fn read_cddb(&mut self, cddb_file: &Path) -> Result<()> {
        let mut cddb = read_cddb(BufReader::new(File::open(cddb_file)?))?;

        self.cddb = cddb.remove("DISCID").map(|c| u32::from_str_radix(&c, 16)).invert()?;
        if let Some(at) = cddb.remove("DTITLE") {
            if let Some((artist, album)) = at.split_once(" / ") {
                self.artist = Some(artist.to_owned());
                self.disc_name = Some(album.to_owned());
            }
        }
        self.date = cddb.remove("DYEAR").map(|y| y.parse()).invert()?;
        self.genre = cddb.remove("DGENRE");

        Ok(())
    }
}
