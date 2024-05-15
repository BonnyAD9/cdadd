use std::{collections::HashMap, io::BufRead};

use crate::err::{Error, Result};

pub fn read_cddb<R>(input: R) -> Result<HashMap<String, String>>
where
    R: BufRead,
{
    let mut res = HashMap::new();

    for l in input.lines() {
        let l = l?;
        if l.is_empty() || l.starts_with('#') {
            continue;
        }

        let Some((key, value)) = l.split_once('=') else {
            return Err(Error::ParseCddb);
        };

        if value.is_empty() {
            continue;
        }

        res.entry(key.to_owned())
            .and_modify(|a: &mut String| a.push_str(value))
            .or_insert(value.to_owned());
    }

    Ok(res)
}
