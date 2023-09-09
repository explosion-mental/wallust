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

    println!("Reading image first..");
    let sample = backends::main(&Backend::Resized)(Path::new(SRC[0])).expect("Download the image {SRC[0]}");
    println!("Done.");

    let mut group = c.benchmark_group("color-spaces");

    let possible_cases = [
        ColorSpaces::Lab,
        ColorSpaces::LabMixed,
    ];

    //for i in SRC {
        let name = SRC[0];
        //let i = Path::new(i);

        for j in possible_cases {
            group.bench_with_input(
                BenchmarkId::new(j.to_string(), &name),
                &sample,
                |b, i| b.iter(|| colorspaces::main(j, i, 20, ColorOrder::DarkFirst))

            );
        }

    group.finish();
}

criterion_group!(benches, colorspaces);
criterion_main!(benches);
