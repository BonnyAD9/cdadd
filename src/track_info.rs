use std::{any::type_name, fmt::Display, path::Path, str::FromStr};

use ini::{Ini, Properties};
use log::{error, warn};

use crate::err::Result;

#[derive(Default, Debug)]
pub struct TrackInfo {
    // album info
    pub cdindex_discid: Option<String>,
    pub cddb_discid: Option<u32>,
    pub album_performer: Option<String>,
    pub album_title: Option<String>,
    pub disc: Option<usize>,

    // track info
    pub isrc: Option<String>,
    pub performer: Option<String>,
    pub track_title: Option<String>,
    pub track_number: Option<usize>,
}

impl TrackInfo {
    pub fn from_file<P>(path: P) -> Result<Self> where P: AsRef<Path> {
        let mut info = Ini::load_from_file(&path)?;

        let Some(inf) = info.section_mut::<String>(None) else {
            warn!("Inf file {:?} doesn't contain any info.", path.as_ref());
            return Ok(Self::default());
        };

        Ok(Self {
            cdindex_discid: Self::get_string(inf, "CDINDEX_DISCID"),
            cddb_discid: Self::get_hex_u32(inf, "CDDB_DISCID"),
            album_performer: Self::get_string(inf, "Albumperformer"),
            album_title: Self::get_string(inf, "Albumtitle"),
            disc: None,

            isrc: Self::get_string(inf, "ISRC"),
            performer: Self::get_string(inf, "Performer"),
            track_title: Self::get_string(inf, "Tracktitle"),
            track_number: Self::get_parse(inf, "Track"),
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
