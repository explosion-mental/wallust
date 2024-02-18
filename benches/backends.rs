use criterion::{criterion_group, criterion_main, Criterion};

use wallust::backends::{self, Backend};

use strum::IntoEnumIterator;

include!("util.in");

fn backends(c: &mut Criterion) {
    let root = get_dir();

    for curr in Backend::iter() {
        let mut group = c.benchmark_group(curr.to_string());

        //iterate over all images
        for image in IMAGES {
            let path = root.join("target").join("benchimg").join(image);

            //with all possible backends
            group.bench_function(
                image,
                |b| b.iter(|| backends::main(&curr)(&path).expect("Download the images"))

            );
        }
    }

}

criterion_group! {
  name = benches;
  config = Criterion::default().sample_size(10);
  targets = backends
}
criterion_main!(benches);
