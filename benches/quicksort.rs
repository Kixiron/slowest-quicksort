use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{distributions::Standard, thread_rng, Rng};
use slowest_quicksort::*;

fn benchmarks(c: &mut Criterion) {
    let rng = thread_rng();

    let vec: Vec<usize> = rng.sample_iter(Standard).take(100_000).collect();
    let (low, high) = (0, vec.len() - 1);

    c.bench_function("Normal Quicksort", |b| {
        b.iter(|| {
            normal::quicksort(black_box(&mut vec.clone()), black_box(low), black_box(high));
        });
    })
    .bench_function("Boxed Quicksort", |b| {
        let vec: Box<Box<Vec<Box<Box<Box<usize>>>>>> = Box::new(Box::new(
            vec.clone()
                .into_iter()
                .map(|elem| Box::new(Box::new(Box::new(elem))))
                .collect(),
        ));

        b.iter(|| {
            boxed::quicksort(
                black_box(&mut vec.clone()),
                black_box(Box::new(Box::new(Box::new(low)))),
                black_box(Box::new(Box::new(Box::new(high)))),
            )
        });
    })
    .bench_function("Allocating Quicksort", |b| {
        let vec: Box<Box<Vec<Box<Box<Box<usize>>>>>> = Box::new(Box::new(
            vec.clone()
                .into_iter()
                .map(|elem| Box::new(Box::new(Box::new(elem))))
                .collect(),
        ));

        b.iter(|| {
            realloc::quicksort(
                black_box(&mut vec.clone()),
                black_box(Box::new(Box::new(Box::new(low)))),
                black_box(Box::new(Box::new(Box::new(high)))),
            )
        });
    })
    .bench_function("Lockful Quicksort", |b| {
        use std::sync::{Arc, Mutex};

        b.iter(|| {
            let vec: Arc<Mutex<Box<Box<Vec<Box<Box<Box<Arc<Mutex<usize>>>>>>>>>> =
                Arc::new(Mutex::new(Box::new(Box::new(
                    vec.clone()
                        .into_iter()
                        .map(|elem| Box::new(Box::new(Box::new(Arc::new(Mutex::new(elem))))))
                        .collect(),
                ))));

            lockful::quicksort(
                black_box(vec),
                black_box(Box::new(Box::new(Box::new(Arc::new(Mutex::new(low)))))),
                black_box(Box::new(Box::new(Box::new(Arc::new(Mutex::new(high)))))),
            )
            .join()
            .unwrap();
        });
    });
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
