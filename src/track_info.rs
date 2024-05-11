use std::{any::type_name, fmt::Display, path::Path, str::FromStr};

use ini::{Ini, Properties};
use log::{error, warn};

use crate::{date::Date, err::Result};

#[derive(Default, Debug)]
pub struct TrackInfo {
    // album info
    pub cdindex: Option<String>,
    pub cddb: Option<u32>,
    pub album_artist: Option<String>,
    pub album: Option<String>,
    pub disc: Option<usize>,
    pub date: Option<Date>,
    pub genre: Option<String>,

    // track info
    pub isrc: Option<String>,
    pub artist: Option<String>,
    pub title: Option<String>,
    pub track: Option<usize>,
}

impl TrackInfo {
    pub fn from_file<P>(path: P) -> Result<Self> where P: AsRef<Path> {
        let mut info = Ini::load_from_file(&path)?;

        let Some(inf) = info.section_mut::<String>(None) else {
            warn!("Inf file {:?} doesn't contain any info.", path.as_ref());
            return Ok(Self::default());
        };

        Ok(Self {
            cdindex: Self::get_string(inf, "CDINDEX_DISCID"),
            cddb: Self::get_hex_u32(inf, "CDDB_DISCID"),
            album_artist: Self::get_string(inf, "Albumperformer"),
            album: Self::get_string(inf, "Albumtitle"),
            disc: None,
            date: None,
            genre: None,

            isrc: Self::get_string(inf, "ISRC"),
            artist: Self::get_string(inf, "Performer"),
            title: Self::get_string(inf, "Tracktitle"),
            track: Self::get_parse(inf, "Track"),
        })
    }

    fn get_string<S>(inf: &mut Properties, name: S) -> Option<String> where S: AsRef<str> {
        if let Some(s) = inf.remove(&name) {
            if s.is_empty() {
                warn!("Value for property '{}' is empty.", name.as_ref());
                None
            } else {
                Some(s)
            }
        } else {
            warn!("Missing property '{}'.", name.as_ref());
            None
        }
    }

    fn get_parse<V, S>(inf: &Properties, name: S) -> Option<V> where S: AsRef<str>, V: FromStr, V::Err: Display {
        if let Some(s) = inf.get(&name) {
            if s.is_empty() {
                warn!("Value for property '{}' is empty.", name.as_ref());
                None
            } else {
                match s.parse::<V>() {
                    Ok(v) => Some(v),
                    Err(e) => {
                        error!("Failed to parse '{s}' into {}: {e}", type_name::<V>());
                        None
                    }
                }
            }
        } else {
            warn!("Missing property '{}'.", name.as_ref());
            None
        }
    }

    fn get_hex_u32<S>(inf: &Properties, name: S) -> Option<u32> where S: AsRef<str> {
        if let Some(mut s) = inf.get(&name) {
            if s.is_empty() {
                warn!("Value for property '{}' is empty.", name.as_ref());
                None
            } else {
                s = if let Some(s) = s.strip_prefix("0x") {
                    s
                } else {
                    s
                };
                match u32::from_str_radix(s, 16) {
                    Ok(v) => Some(v),
                    Err(e) => {
                        error!("Failed to parse '{s}' into u32: {e}");
                        None
                    }
                }
            }
        } else {
            warn!("Missing property '{}'.", name.as_ref());
            None
        }
    }
}
