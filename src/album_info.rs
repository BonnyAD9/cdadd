use std::{fs, path::{Path, PathBuf}};

use log::warn;

use crate::{err::Result, track_info::TrackInfo};

#[derive(Default, Debug)]
pub struct AlbumInfo {
    pub cdindex: Option<String>,
    pub cddb: Option<u32>,
    pub artist: Option<String>,
    pub title: Option<String>,
    pub disc: Option<usize>,
    pub year: Option<i32>,
    pub genre: Option<String>,

    pub tracks: Vec<(TrackInfo, PathBuf)>,
}

impl AlbumInfo {
    pub fn from_dir<P>(path: P) -> Result<Self> where P: AsRef<Path> {
        let mut res = Self::default();
        res.load_dir(path.as_ref())?;
        Ok(res)
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

        self.cdindex = self.tracks.iter().flat_map(|(t, _)| t.cdindex.clone()).next();
        self.cddb = self.tracks.iter().flat_map(|(t, _)| t.cddb).next();
        self.artist = self.tracks.iter().flat_map(|(t, _)| t.album_artist.clone()).chain(self.tracks.iter().flat_map(|(t, _)| t.artist.clone())).next();
        self.title = self.tracks.iter().flat_map(|(t, _)| t.album.clone()).next();
        self.disc = self.tracks.iter().flat_map(|(t, _)| t.disc).next();
        self.year = self.tracks.iter().flat_map(|(t, _)| t.year).next();
        self.genre = self.tracks.iter().flat_map(|(t, _)| t.genre.clone()).next();

        for (t, _) in self.tracks.iter_mut() {
            t.cdindex = t.cdindex.take().or_else(|| self.cdindex.clone());
            t.cddb = t.cddb.or(self.cddb);
            t.artist = t.artist.take().or_else(|| self.artist.clone());
            t.album = t.album.take().or_else(|| self.title.clone());
            t.disc = t.disc.or(self.disc);
            t.year = t.year.or(self.year);
            t.genre = t.genre.take().or_else(|| self.genre.clone());
        }

        Ok(())
    }
}
