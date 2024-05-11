use std::{fs, path::{Path, PathBuf}};

use log::warn;

use crate::{err::Result, track_info::TrackInfo};

#[derive(Default, Debug)]
pub struct AlbumInfo {
    pub cdindex_discid: Option<String>,
    pub cddb_discid: Option<u32>,
    pub album_performer: Option<String>,
    pub album_title: Option<String>,
    pub disc: Option<usize>,

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

        self.tracks.sort_by_key(|t| t.0.track_number);

        Ok(())
    }
}
