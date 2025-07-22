//! Cache functions, serde + serde_json
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use palette::Srgb;

use crate::colors::Colors;
use crate::config::Config;

use anyhow::{Result, Context};

/// Cache versioning, to avoid breaks and missreadings.
/// For example, when there is an internal change in how the
/// scheme is generated, the cache format won't change, however,
/// there is a need for a regeneration, so we bump up the version.
pub const CACHE_VER: &str = "1.7";

/// Used to manage cache, rather than passing arguments in main() a lot
#[derive(Debug, Default)]
pub struct Cache {
    /// Path of the cache, this is the path read.
    pub path: PathBuf,
    /// backend file, doesn't include de thereshold since it doesn't affects it
    pub back: PathBuf,
    /// colorscace file + threshold
    pub cs: PathBuf,
    /// palette file + threshold
    pub palette: PathBuf,
    /// preset cache
    pub preset: Option<PathBuf>,

    /// Path name
    pub name: PathBuf,
}

/// Simply print the path when trying to display the [`Cache`] struct
impl fmt::Display for Cache {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.path.display())
    }
}

/// Simple shadow for colorscheme return type
type CSret = (Vec<Srgb>, Vec<Srgb>, bool);

/// Cache order
#[derive(Debug)]
pub enum IsCached {
    None,
    Backend,
    BackendnCS,
    BackendnCSnPalette,
    Preset,
}

impl Cache {
    /// # Filename structure, magic numbers (cachefmt) after this impl block:
    /// *Each hash image has it's own dir*, inside there is multiple files:
    /// 1. Backend file, with the full name, maybe reductant with the `full` backend.
    /// 2. ColorSpace + threshold, since it depends on the threshold
    /// 3. Scheme + ColorSpace + threshold, since the palette depends on the colorspace, and the colorspace on the threshold
    ///    This new structure allows you to reuse some parts, when configuring, avoding more time waiting.
    pub fn new(file: &Path, c: &Config, cache_path: &Path) -> Result<Self> {
        // create cache (e.g. `~/.cache/wallust`)
        let cachepath = cache_path.join("wallust");

        // hash value for the file, since you can duplicate it, but the contents are the same.
        let hash  = base36(fnv1a(&std::fs::read(file)?));

        let name = cachepath.join(format!("{hash}_{CACHE_VER}"));
        // Create cache dir (with all of it's parents)
        fs::create_dir_all(&name).with_context(|| "Failed to create {cachepath}")?;

        let th    = if c.true_th == 0 { "auto" } else { &c.true_th.to_string() };
        // wallust/image_1.0/
        let base = cachepath.join(format!("{hash}_{CACHE_VER}"));

        let back = c.backend.to_string();
        let cs  = c.color_space.to_string();
        let palet = c.palette.to_string();
        let preset = match &c.preset {
            Some(s) => Some(base.join(s.to_string())),
            None => None,
        };

        Ok(Self {
            path: cachepath,
            name,
            back: base.join(&back),
            cs: base.join(format!("{back}_{cs}_{th}")),
            palette: base.join(format!("{back}_{cs}_{th}_{palet}")),
            preset,
        })
    }

    // Update path
    // pub fn reached_gen(&mut self) {
    //     self.path.clone_from(&self.gen);
    // }

    pub fn read_backend(&self) -> Result<Vec<u8>> {
        let contents = std::fs::read_to_string(&self.back)?;
        let v: Vec<u8> = serde_json::from_str(&contents)?;
        Ok(v)
    }

    pub fn read_cs(&self) -> Result<CSret> {
        let contents = std::fs::read_to_string(&self.cs)?;
        let v: CSret = serde_json::from_str(&contents)?;
        Ok(v)
    }

    pub fn read_palette(&self) -> Result<Colors> {
        let contents = std::fs::read_to_string(&self.palette)?;
        let v: Colors = serde_json::from_str(&contents)?;
        Ok(v)
    }

    pub fn read_preset(&self) -> Result<Colors> {
        let p = self.preset.as_ref().expect("Only called inside lib.rs");
        let contents = std::fs::read_to_string(p)?;
        let v: Colors = serde_json::from_str(&contents)?;
        Ok(v)
    }

    /// XXX Given that presets edit out the ColorSpace part, just store the colors.
    pub fn write_preset(&self, c: &Colors) -> Result<()> {
        let p = self.preset.as_ref().expect("Only called inside lib.rs");
        Ok(File::create(p)?
            .write_all(
                serde_json::to_string_pretty(c)
                    .with_context(|| format!("Failed to deserilize from the json cached file: '{}':", &self))?
                .as_bytes()
            )?
        )
    }

    pub fn write_backend(&self, bytes: &[u8]) -> Result<()> {
        Ok(File::create(&self.back)?
            .write_all(
                serde_json::to_string(bytes)
                    .with_context(|| format!("Failed to deserilize from the json cached file: '{}':", &self))?
                .as_bytes()
            )?
        )
    }

    pub fn write_cs(&self, colorspaces: &CSret) -> Result<()> {
        Ok(File::create(&self.cs)?
            .write_all(
                serde_json::to_string(colorspaces)
                    .with_context(|| format!("Failed to deserilize from the json cached file: '{}':", &self))?
                .as_bytes()
            )?
        )
    }

    pub fn write_palette(&self, scheme: &Colors) -> Result<()> {
        Ok(File::create(&self.palette)?
            .write_all(
                serde_json::to_string_pretty(scheme)
                    .with_context(|| format!("Failed to deserilize from the json cached file: '{}':", &self))?
                .as_bytes()
            )?
        )
    }

    pub fn is_cached_all(&self) -> IsCached {
        match self.preset {
            Some(_) => return IsCached::Preset,
            None => (),
        }

        let b  = self.back.exists();
        let cs = self.cs.exists();
        let p  = self.palette.exists();

        if b && cs && p {
            IsCached::BackendnCSnPalette
        } else if b && cs {
            IsCached::BackendnCS
        } else if b {
            IsCached::Backend
        } else {
            IsCached::None
        }
    }
}

/* helpers */

/// Pretty fcking fast hashing
/// the 32 bit version, should be enough for this use case
/// Ref: https://en.m.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function
pub fn fnv1a(bytes: &[u8]) -> u32 {
    let mut hash = 2166136261;

    for byte in bytes {
        hash ^= *byte as u32;
        hash = hash.wrapping_mul(16777619);
    }

    hash
}

/// simple base36 encoding
/// Also, there is no need to decode, since it should match if the contents of the file are the
/// same, else just generate a new scheme.
/// ref: https://stackoverflow.com/questions/50277050/format-convert-a-number-to-a-string-in-any-base-including-bases-other-than-deci
pub fn base36(n: u32) -> String {
    let mut n = n;
    let mut result = vec![];

    loop {
        let m = n % 36;
        n /= 36;
        result.push(std::char::from_digit(m, 36).expect("is between [2; 36]"));
        if n == 0 { break; }
    }
    result.into_iter().rev().collect()
}
