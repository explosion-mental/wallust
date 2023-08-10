//! Cache functions, serde + serde_json
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

#[cfg(windows)]
use std::os::windows::fs::MetadataExt;

use crate::colors::Colors;
use crate::config::Config;

use anyhow::{Result, Context};

/// Used to manage cache, rather than passing arguments in main() a lot
pub struct Cache {
    /// Path of the cache
    pub path: String,
}

const CACHE_VER: &str = "1.0";

impl Cache {
    /// init cache
    pub fn new(filename: &Path, c: &Config, cache_path: &Path) -> Result<Self> {

        let Some(name) = filename.file_name() else {
            anyhow::bail!("Using '..' as a parameter is not supported");
        };

        let cachepath = format!("{root}/wallust/{back}/{th}/{cs}/{filter}",
            root = cache_path.display(), // ~/.cache/
            back = c.backend,
            th = c.threshold,
            cs = c.color_space,
            filter = c.filter,
        );

        // Create cache dir (with all of it's parents)
        fs::create_dir_all(&cachepath)?;

        // get medatada
        let md = fs::metadata(filename)?;

        // use the ino number on *nix systems, and the "magick file number" on windows
        #[cfg(unix)]
        let num = md.ino();
        #[cfg(windows)]
        let num = md.file_attributes() ;

        // The following generates a hash name from a filename and it's `stat` attrs
        let hash_name = format!("{}_{}_{}_{}.json",
            name.to_string_lossy(),
            md.len(),
            num,
            CACHE_VER,
        );

        Ok(Self { path: format!("{cachepath}/{hash_name}") })
    }

    /// Fetches values from a file present in cache
    pub fn read(&self) -> Result<Colors> {
        let contents = std::fs::read_to_string(&self.path)?;
        Ok(serde_json::from_str(&contents)?)
    }

    /// Write values to cache
    pub fn write(&self, colors: &Colors) -> Result<()> {
        Ok(File::create(&self.path)?
            .write_all(
                serde_json::to_string(colors)
                    .with_context(|| format!("Failed to deserilize from the json cached file: '{}':", &self.path))?
                .as_bytes()
            )?
        )
    }

    /// To determine whether to read from cache or to generate the colors from scratch
    pub fn is_cached(&self) -> bool {
        Path::new(&self.path).exists()
    }
}

