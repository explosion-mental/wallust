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

/// Pretty fcking fast hashing
/// the 32 bit version, should be enough for this use case
/// Ref: https://en.m.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function
pub fn fnv1a(bytes: &[u8]) -> usize {
    let mut hash = 2166136261;

    for byte in bytes {
        hash ^= *byte as usize;
        hash = hash.wrapping_mul(16777619);
    }

    hash
}

impl Cache {
    /// # Cache directory structure (using fs as a "database")
    ///   1. Root, determined by OS
    ///   2. "wallust"
    ///   3. backend
    ///   4. colorspace
    ///   5. palette
    ///   6. threshold
    ///   7. saturation percentage (OPTIONAL)
    /// # Filename structure:
    ///   1. hash
    ///   2. inode number on Linux, file attributes on Windows
    ///   3. check-contrast -> "C_" if true, "" if false (OPTIONAL)
    ///   4. fallback generator (if reached)
    ///   4. [`CACHE_VER`]
    pub fn new(file: &Path, c: &Config, cache_path: &Path) -> Result<Self> {
        // A possible solution to caching a checked/unchecked contrast without cache duplication and
        // possible efficiency loss
        // enum Contrast {
        //     Checked,
        //     Unchecked,
        //     UncheckedAndGood,
        // }

        let sat = if let Some(s) = c.saturation {
            format!("saturation-{s}")
        } else {
            "".to_string()
        };

        // threshold
        let th = if c.true_th == 0 { "auto" } else { &c.true_th.to_string() };

        // Cache directory structure
        let cachepath = Path::new(cache_path)
            .join("wallust")
            .join(c.backend.to_string())
            .join(c.color_space.to_string())
            .join(c.palette.to_string())
            .join(th)
            .join(sat)
        ;

        // Create cache dir (with all of it's parents)
        fs::create_dir_all(&cachepath)?;

        #[cfg(unix)] // use the ino number on *nix systems
        let num = fs::metadata(file)?.ino();

        #[cfg(windows)] // and the "magick file number" on windows
        let num = fs::metadata(file)?.file_attributes();

        // Filename structure
        let basename = format!("{hash}_{magic}_{con}{version}",
            hash  = fnv1a(&std::fs::read(file)?),
            magic = num,
            con   = if c.check_contrast.unwrap_or(false) { "C_" } else { "" },
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
        self.path.clone_from(&self.gen);
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

