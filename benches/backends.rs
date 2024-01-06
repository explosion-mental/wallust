use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use wallust::backends::{self, Backend};
use std::path::Path;

const SRC: [&str; 4] = [
    "pexels-photo-356036.jpeg",
    "pexels-photo-1146708.jpeg",
    "pexels-photo-1567069.jpeg",
    "pexels-photo-1089194.jpeg",
];

fn backends(c: &mut Criterion) {
    let mut group = c.benchmark_group("backends");

    let home = std::process::Command::new("cargo").arg("locate-project").output().unwrap();
    let home = std::str::from_utf8(&home.stdout).unwrap();
    let root: serde_json::Value = serde_json::from_str(home).unwrap();
    let root = root.pointer("/root").unwrap();
    let root: String = serde_json::from_value(root.clone()).unwrap();
    let root = &root[0..root.len() - "Cargo.toml".len()];

    let possible_cases = [
        Backend::Full,
        Backend::Resized,
        Backend::Thumb,
        Backend::Wal,
        Backend::FastResize,
    ];

    //iterate over all images
    for i in SRC {
        let name = i;
        let i = &Path::new(&root).join("target").join("benchimg").join(i);

        //with all possible backends
        for j in possible_cases {
            group.bench_with_input(
                BenchmarkId::new(j.to_string(), &name),
                i,
                |b, i| b.iter(|| backends::main(&j)(i).expect("Download the images"))

            );
        }
    }

    group.finish();
}

criterion_group! {
  name = benches;
  config = Criterion::default().sample_size(10);
  targets = backends
}
criterion_main!(benches);
