//! Cache functions, stored in json
use std::path::PathBuf;

use crate::Colors;
use crate::MyLab;

use anyhow::Result;

/// Writes data to a file in cache.
//TODO to overwrite or to check if it exist first? maybe a cli flag to overwrite/delete cache
pub fn write_cache(colors: Colors<MyLab>, file: &PathBuf) -> Result<()> {
    if already_cached(file)? { return Ok(()); }
    println!("{}", serde_json::to_string(&colors)?);
    Ok(())
}

/// Loads data from a cached file
pub fn read_cache() -> Result<Colors<MyLab>> {
    let path = "~/.cache/wallust";
    let contents = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&contents)?)
}

/// Check if there is cached data for the `f`ile given
pub fn already_cached(f: &PathBuf) -> Result<bool> {
    Ok(true)
}

/// Generates the name the cache file it's gonna use, which is a hash
fn cachename(file: &PathBuf) -> Result<String> {
    Ok("".into())
}
