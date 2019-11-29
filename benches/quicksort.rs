use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{distributions::Standard, thread_rng, Rng};
use slowest_quicksort::{boxed, locked_no_threads, lockful, normal, realloc};
use std::sync::{Arc, Mutex};

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
        b.iter(|| {
            let vec: Box<Box<Vec<Box<Box<Box<usize>>>>>> = Box::new(Box::new(
                vec.clone()
                    .into_iter()
                    .map(|elem| Box::new(Box::new(Box::new(elem))))
                    .collect(),
            ));

            boxed::quicksort(
                black_box(&mut vec.clone()),
                black_box(Box::new(Box::new(Box::new(low)))),
                black_box(Box::new(Box::new(Box::new(high)))),
            )
        });
    })
    .bench_function("Allocating Quicksort", |b| {
        b.iter(|| {
            let vec: Box<Box<Vec<Box<Box<Box<usize>>>>>> = Box::new(Box::new(
                vec.clone()
                    .into_iter()
                    .map(|elem| Box::new(Box::new(Box::new(elem))))
                    .collect(),
            ));

            realloc::quicksort(
                black_box(&mut vec.clone()),
                black_box(Box::new(Box::new(Box::new(low)))),
                black_box(Box::new(Box::new(Box::new(high)))),
            )
        });
    })
    .bench_function("Lockful Quicksort", |b| {
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
    })
    .bench_function("Threadpool Quicksort", |b| {
        b.iter(|| {
            let vec: Arc<Mutex<Box<Box<Vec<Box<Box<Box<Arc<Mutex<usize>>>>>>>>>> =
                Arc::new(Mutex::new(Box::new(Box::new(
                    vec.clone()
                        .into_iter()
                        .map(|elem| Box::new(Box::new(Box::new(Arc::new(Mutex::new(elem))))))
                        .collect(),
                ))));

            let pool = threadpool::Builder::new()
                .num_threads(1)
                .thread_stack_size(8_000_000)
                .build();

            slowest_quicksort::threadpool::quicksort(
                black_box(vec),
                black_box(Box::new(Box::new(Box::new(Arc::new(Mutex::new(low)))))),
                black_box(Box::new(Box::new(Box::new(Arc::new(Mutex::new(high)))))),
                black_box(pool),
            );
        })
    })
    .bench_function("Threadless Locked Quicksort", |b| {
        b.iter(|| {
            let vec: Arc<Mutex<Box<Box<Vec<Box<Box<Box<Arc<Mutex<usize>>>>>>>>>> =
                Arc::new(Mutex::new(Box::new(Box::new(
                    vec.clone()
                        .into_iter()
                        .map(|elem| Box::new(Box::new(Box::new(Arc::new(Mutex::new(elem))))))
                        .collect(),
                ))));

            locked_no_threads::quicksort(
                black_box(vec),
                black_box(Box::new(Box::new(Box::new(Arc::new(Mutex::new(low)))))),
                black_box(Box::new(Box::new(Box::new(Arc::new(Mutex::new(high)))))),
            );
        });
    });
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
