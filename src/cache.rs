//! Cache functions, stored in json
//!
//! You either write_cache() or read_cache()
use std::path::PathBuf;
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::time::SystemTime;

use crate::Colors;
use crate::MyLab;

use anyhow::Result;

/// Writes data to a file in cache.
//TODO to overwrite or to check if it exist first? maybe a cli flag to overwrite/delete cache
pub fn write_cache(colors: Colors<MyLab>, file: &PathBuf) -> Result<()> {
    if already_cached(file)? { return Ok(()); }
    let cname = cachename(file);
    println!("{}", serde_json::to_string(&colors)?);
    Ok(())
}

/// Loads data from a cached file
pub fn read_cache(file: &PathBuf) -> Result<Colors<MyLab>> {
    let path = "~/.cache/wallust";
    let cname = cachename(file);
    let contents = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&contents)?)
}

/// Check if there is cached data for the `f`ile given
pub fn already_cached(f: &PathBuf) -> Result<bool> {
    Ok(true)
}

/// Generates the name the cache file it's gonna use, which is a hash
fn cachename(file: &PathBuf) -> Result<String> {
    let md = fs::metadata(&file)?;
    let birth = if let Ok(o) = md.created() { o } else { panic!("Not Supported") };
    let modif = if let Ok(o) = md.modified() { o } else { panic!("Not Supported") };
    Ok(
    format!("{}{}{}{}{}",
        file.display(),
        md.ino(),
        //md.file_type(),
        md.len(),
        birth.duration_since(SystemTime::UNIX_EPOCH)?.as_secs(),
        modif.duration_since(SystemTime::UNIX_EPOCH)?.as_secs(),
    ))
}
