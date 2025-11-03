//! Cache functions, serde + serde_json
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::BufReader;
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

    pub fn read_backend(&self) -> Result<Vec<u8>> { read_json(&self.back) }
    pub fn read_cs(&self) -> Result<CSret> { read_json(&self.cs) }
    pub fn read_palette(&self) -> Result<Colors> { read_json(&self.palette) }

    pub fn read_preset(&self) -> Result<Colors> {
        let p = self.preset.as_ref().expect("Only called inside lib.rs"); // TODO
        read_json(p)
    }

    /// XXX Given that presets edit out the ColorSpace part, just store the colors.
    pub fn write_preset(&self, c: &Colors) -> Result<()> {
        let p = self.preset.as_ref().expect("Only called inside lib.rs"); //TODO avoid this
        write_json(p, c, &self.to_string(), true)
    }

    pub fn write_backend(&self, bytes: &[u8]) -> Result<()> { write_json(&self.back, &bytes, &self.to_string(), false) }
    pub fn write_cs(&self, colorspaces: &CSret) -> Result<()> { write_json(&self.cs, colorspaces, &self.to_string(), false) }
    pub fn write_palette(&self, scheme: &Colors) -> Result<()> { write_json(&self.palette, scheme, &self.to_string(), true) }

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

/// path is the new location of the file to be written to
/// value the contents of it, `cachepath` is only to print the cache absolute path,
/// and pretty to use serde_to_string_pretty
fn write_json<P: AsRef<std::path::Path>, T: serde::Serialize>(path: P, value: &T, cachepath: &str, pretty: bool) -> anyhow::Result<()> {
    let serde_to_string = if pretty { serde_json::to_string_pretty } else { serde_json::to_string };
    Ok(File::create(&path)?
        .write_all(
            serde_to_string(value)
            .with_context(|| format!("Failed to deserilize from the json cached file: '{cachepath}':"))?
            .as_bytes()
        )?
    )
}

fn read_json<P: AsRef<std::path::Path>, T: serde::de::DeserializeOwned>(path: P) -> anyhow::Result<T> {
    let path = path.as_ref();
    let f = File::open(path).with_context(|| format!("Failed to open cache file '{}'", path.display()))?;
    serde_json::from_reader(BufReader::new(f))
        .with_context(|| format!("Failed to parse JSON in cache file '{}'", path.display()))
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
