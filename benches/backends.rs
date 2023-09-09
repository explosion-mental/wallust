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
    // download the image, since it's very large and adding it to the git repo wouldn't be feasible

    // let urls = vec![
    // //Photo by Pixabay from Pexels: https://www.pexels.com/photo/blue-solar-panel-board-356036/
    // "https://images.pexels.com/photos/356036/pexels-photo-356036.jpeg",
    //
    // //Photo by Johannes Plenio from Pexels: https://www.pexels.com/photo/green-rice-field-1146708/
    // "https://images.pexels.com/photos/1146708/pexels-photo-1146708.jpeg",
    //
    // //Photo by Yuting Gao from Pexels: https://www.pexels.com/photo/silhouette-of-two-persons-stargazing-1567069/
    // "https://images.pexels.com/photos/1567069/pexels-photo-1567069.jpeg",
    //
    // //Photo by Yuting Gao from Pexels: https://www.pexels.com/photo/stranger-things-2-sign-in-city-at-night-1089194/
    // "https://images.pexels.com/photos/1089194/pexels-photo-1089194.jpeg"];
    //
    // for i in urls {
    //     Command::new("wget").arg(i).spawn().unwrap();
    // }

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
