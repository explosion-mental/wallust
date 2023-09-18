use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

use wallust::backends::{self, Backend};
use wallust::colorspaces::{self, ColorSpaces, ColorOrder};

use std::path::Path;

const SRC: [&str; 4] = [
    "pexels-photo-356036.jpeg",
    "pexels-photo-1146708.jpeg",
    "pexels-photo-1567069.jpeg",
    "pexels-photo-1089194.jpeg",
];

fn colorspaces(c: &mut Criterion) {
    let mut group = c.benchmark_group("color-spaces");

    let home = std::process::Command::new("cargo").arg("locate-project").output().unwrap();
    let home = std::str::from_utf8(&home.stdout).unwrap();
    let root: serde_json::Value = serde_json::from_str(home).unwrap();
    let root = root.pointer("/root").unwrap();
    let root: String = serde_json::from_value(root.clone()).unwrap();
    let root = &root[0..root.len() - "Cargo.toml".len()];

    let possible_cases = [
        ColorSpaces::Lab,
        ColorSpaces::LabMixed,
        ColorSpaces::LabFast,
    ];

    for i in SRC {
        let name = i;

        println!("Reading image first.. {i}");
        let p = &Path::new(&root).join("target").join("benchimg").join(i);
        let sample = backends::main(&Backend::Resized)(Path::new(p)).expect(&format!("Download the image {i}"));
        println!("Done.\n");

        for j in possible_cases {
            group.bench_with_input(
                BenchmarkId::new(j.to_string(), &name),
                &sample,
                |b, i| b.iter(|| colorspaces::main(j, i, 20, ColorOrder::DarkFirst))

            );
        }
    }

    group.finish();
}

criterion_group!(benches, colorspaces);
criterion_main!(benches);
