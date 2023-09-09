use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use wallust::backends::{self, Backend};
use std::path::Path;

const SRC: [&str; 1] = [
    "pexels-photo-356036.jpeg",
    //"pexels-photo-1146708.jpeg",
    //"pexels-photo-1567069.jpeg",
    //"pexels-photo-1089194.jpeg",
];

fn backends(c: &mut Criterion) {
    let mut group = c.benchmark_group("backends");

    let possible_cases = [
        Backend::Full,
        Backend::Resized,
        Backend::Thumb,
        Backend::Wal,
    ];

    //iterate over all images
    for i in SRC {
        let name = i;
        let i = Path::new(i);

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

criterion_group!(benches, backends);
criterion_main!(benches);
