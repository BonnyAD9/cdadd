use crate::err::{Error, Result};

pub fn get_perf(s: &str) -> Result<Vec<String>> {
    let Some((_, perf)) = s.split_once("(feat. ") else {
        return Ok(vec![]);
    };

    let Some((perf, _)) = perf.split_once(')') else {
        return Err(Error::ParseFeat("Missing closing ')'"));
    };

    Ok(perf
        .split('&')
        .flat_map(|p| p.split('&'))
        .map(|s| s.trim().to_owned())
        .collect())
}
