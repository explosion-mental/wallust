//! Cache functions, serde + serde_json
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use crate::colors::Colors;
use crate::backends::Backend;
use crate::palettes::Palette;
use crate::colorspaces::ColorSpace;
use crate::colorspaces::FallbackGenerator;
use crate::config::Config;

use anyhow::{Result, Context};

/// Cache versioning, to avoid breaks and missreadings.
pub const CACHE_VER: &str = "1.5";

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

impl Cache {
    /// # Filename structure, magic numbers (cachefmt) after this impl block:
    ///   1. hash base36 encoded
    ///   2. CACHE_VER
    ///   3. backend
    ///   4. colorspace
    ///   5. palette
    ///   6. threshold -> "0" means auto, else it's the threshold assign
    ///   7. saturation percentage -> either it's percentage or "0" if it's disabled
    ///   8. check-contrast -> 0 off, 1 on
    ///   9. fallback generator
    pub fn new(file: &Path, c: &Config, cache_path: &Path) -> Result<Self> {
        // create cache (e.g. `~/.cache/wallust`)
        let cachepath = cache_path.join("wallust");

        // Create cache dir (with all of it's parents)
        fs::create_dir_all(&cachepath).with_context(|| "Failed to create {cachepath}")?;

        //XXX maybe later just use an [String; 8].join("_")

        let hash  = base36(fnv1a(&std::fs::read(file)?));
        let back  = c.backend.cachefmt();
        let cs    = c.color_space.cachefmt();
        let palet = c.palette.cachefmt();
        let th    = if c.true_th == 0 { "0" } else { &c.true_th.to_string() };
        let sat   = if let Some(s) = c.saturation { s.to_string() } else { "0".to_owned() };
        let con   = if c.check_contrast.unwrap_or(false) { "1" } else { "0" };

        let gen = "0"; //asume it didn't reached a fallback generator
        let basename   = format!("{CACHE_VER}_{back}_{cs}_{palet}_{th}_{sat}_{con}_{gen}_{hash}");

        let gen = c.fallback_generator.unwrap_or_default().cachefmt().to_string(); // real fallback generator
        let generation = format!("{CACHE_VER}_{hash}_{cs}_{palet}_{th}_{sat}_{con}_{gen}_{hash}");

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

    /// Update path
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

/* cache fmts, "magic numbers" */
impl Backend {
    pub fn cachefmt(&self) -> &str {
        match self {
            Backend::Full       => "0",
            Backend::Resized    => "1",
            Backend::Wal        => "2",
            Backend::Thumb      => "3",
            Backend::FastResize => "4",
            Backend::Kmeans     => "5",
        }
    }
}

impl ColorSpace {
    pub fn cachefmt(&self) -> &str {
        match self {
            ColorSpace::Lab      => "0",
            ColorSpace::LabMixed => "1",
            ColorSpace::Lch      => "2",
            ColorSpace::LchMixed => "3",
            ColorSpace::LchAnsi  => "4",
        }
    }
}


impl Palette {
    /// double digits, just to keep a uniform look
    pub fn cachefmt(&self) -> &str {
        match self {
            Palette::Dark            => "00" ,
            Palette::Dark16          => "01" ,
            Palette::DarkComp        => "02" ,
            Palette::DarkComp16      => "03" ,
            Palette::AnsiDark        => "04" ,
            Palette::HardDark        => "05" ,
            Palette::HardDark16      => "06" ,
            Palette::HardDarkComp    => "07" ,
            Palette::HardDarkComp16  => "08" ,
            Palette::Light           => "09" ,
            Palette::Light16         => "10",
            Palette::LightComp       => "11",
            Palette::LightComp16     => "12",
            Palette::SoftDark        => "13",
            Palette::SoftDark16      => "14",
            Palette::SoftDarkComp    => "15",
            Palette::SoftDarkComp16  => "16",
            Palette::SoftLight       => "17",
            Palette::SoftLight16     => "18",
            Palette::SoftLightComp   => "19",
            Palette::SoftLightComp16 => "20",
        }
    }
}

impl FallbackGenerator {
    /// starts from 1, given that 0 means it DIDNT REACHED a fallback generator,
    /// the scheme generated fine.
    pub fn cachefmt(&self) -> &str {
        match self {
            FallbackGenerator::Interpolate => "1",
            FallbackGenerator::Complementary => "2",
        }
    }
}
