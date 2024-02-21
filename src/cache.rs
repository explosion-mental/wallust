//! Cache functions, serde + serde_json
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

#[cfg(windows)]
use std::os::windows::fs::MetadataExt;

use crate::colors::Colors;
use crate::config::Config;

use anyhow::{Result, Context};

/// Used to manage cache, rather than passing arguments in main() a lot
#[derive(Debug, Default)]
pub struct Cache {
    /// The usual naming
    pub normal: PathBuf,
    /// naming with when artificially generating colors
    pub gen: PathBuf,

    /// Path of the cache, this is the path read.
    pub path: PathBuf,
}

/// Simply print the path when trying to display the [`Cache`] struct
impl fmt::Display for Cache {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.path.display())
    }
}

pub const CACHE_VER: &str = "1.4";

impl Cache {
    /// # Cache directory structure
    ///   1. Root, determined by OS
    ///   2. "wallust"
    ///   3. backend
    ///   4. colorspace
    ///   5. palette
    ///   6. threshold
    ///   7. saturation percentage (OPTIONAL)
    /// # File structure:
    ///   1. filename (no extentions)
    ///   2. size
    ///   3. inode number on Linux, file attributes on Windows
    ///   4. check-contrast -> "C_" if true, "" if false
    ///   5. [`CACHE_VER`]
    pub fn new(filename: &Path, c: &Config, cache_path: &Path) -> Result<Self> {


        // A possible solution to caching a checked/unchecked contrast without cache duplication and
        // possible efficiency loss
        // enum Contrast {
        //     Checked,
        //     Unchecked,
        //     UncheckedAndGood,
        // }

        let Some(name) = filename.file_name() else {
            anyhow::bail!("Using '..' as a parameter is not supported");
        };

        let sat = if let Some(s) = c.saturation {
            format!("saturation-{s}")
        } else {
            "".to_string()
        };


        //format!("{root}/wallust/{back}/{th}/{cs}/{palette}",
        let cachepath = Path::new(cache_path)
            .join("wallust")
            .join(c.backend.to_string())
            .join(c.color_space.to_string())
            .join(c.palette.to_string())
            .join(c.threshold.to_string())
            .join(sat)
        ;

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
        let basename = format!("{base}_{size}_{magic}_{con}{version}",
            base = name.to_string_lossy(),
            size = md.len(),
            magic = num,
            con = if c.check_contrast.unwrap_or(false) { "C_" } else { "" },
            version = CACHE_VER,
        );

        let gen_letter = match c.fallback_generator.unwrap_or_default() {
            crate::colorspaces::FallbackGenerator::Interpolate => 'I',
            crate::colorspaces::FallbackGenerator::Complementary => 'C',
        };

        let generation = format!("{basename}_{gen_letter}");

        Ok(Self {
            normal: cachepath.join(basename + ".json"),
            gen:  cachepath.join(generation + ".json"),
            path: PathBuf::new(),

        })
    }

    /// Fetches values from a file present in cache
    pub fn read(&self) -> Result<Colors> {
        let contents = std::fs::read_to_string(&self.path)?;
        Ok(serde_json::from_str(&contents)?)
    }

    pub fn reached_gen(&mut self) {
        self.path = self.gen.clone();
    }

    /// Write values to cache
    pub fn write(&self, colors: &Colors) -> Result<()> {
        Ok(File::create(&self.path)?
            .write_all(
                serde_json::to_string_pretty(colors)
                    .with_context(|| format!("Failed to deserilize from the json cached file: '{}':", &self))?
                .as_bytes()
            )?
        )
    }

    /// To determine whether to read from cache or to generate the colors from scratch
    /// If not found, check if the generated path exist, it could may be that it doesn't have
    /// enought colors.
    pub fn is_cached(&mut self) -> bool {
        let normal = self.normal.exists();
        let gen = self.gen.exists();

        let (new_path, ret) = match (normal, gen) {
            //some exist, so `is_cached()` true
            (true, false) => (self.normal.clone(), true),
            (false, true) => (self.gen.clone(), true),

            // none cached, default to normal and `is_cached()` false
            (false, false) => (self.normal.clone(), false),

            // unusual (imposible?) case. Just default to normal path.
            // if the code reaches the generation part, `reached_gen()` should be called anyway.
            (true,  true)   => (self.normal.clone(), true),
        };

        self.path = new_path;
        ret
    }
}

