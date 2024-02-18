use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

use wallust::backends::{self, Backend};
use wallust::colorspaces::{self, ColorSpace};

use strum::IntoEnumIterator;

include!("util.in");

fn colorspaces(c: &mut Criterion) {
    let root = get_dir();
    let path = root.join("target").join("benchimg");
    let threshold = 20;

    let read_images = IMAGES.map(|x| backends::main(&Backend::Full)(&path.join(x)).unwrap());

    for curr in ColorSpace::iter() {
        let mut group = c.benchmark_group(curr.to_string());

        for (idx, image) in read_images.iter().enumerate() {
            group.bench_with_input(
                BenchmarkId::new(IMAGES[idx], image.len()),
                &image,
                |b, i| b.iter(|| colorspaces::main(curr, i, threshold, &colorspaces::Generate::default()))

            );
        }
    }
}

criterion_group! {
  name = benches;
  config = Criterion::default().sample_size(10);
  targets = colorspaces
}
criterion_main!(benches);
