pub mod normal {
    pub fn quicksort(mut vec: &mut Vec<usize>, low: usize, high: usize) {
        if low < high {
            // Choose a pivot
            let pivot = partition(&mut vec, low, high);

            // Sort the lower sub-array
            quicksort(&mut vec, low, pivot.saturating_sub(1));

            // Sort the higher sub-array
            quicksort(&mut vec, pivot + 1, high);
        }
    }

    fn partition(vec: &mut Vec<usize>, low: usize, high: usize) -> usize {
        let pivot = vec[high];
        let mut i = low;

        // For each element in the sub-array
        for j in low..high {
            // If the item is less than the pivot
            if vec[j] < pivot {
                // Swap the i'th and j'th item
                vec.swap(i, j);
                // Increment i
                i += 1;
            }
        }

        // Swap the highest element and the i'th element
        vec.swap(i, high);

        // Return the pivot point
        i
    }
}

pub mod boxed {
    pub fn quicksort(
        mut vec: &mut Box<Box<Vec<Box<Box<Box<usize>>>>>>,
        low: Box<Box<Box<usize>>>,
        high: Box<Box<Box<usize>>>,
    ) {
        if low < high {
            // Choose a pivot
            let pivot = partition(&mut vec, low.clone(), high.clone());

            // Sort the lower sub-array
            quicksort(
                &mut vec,
                low.clone(),
                Box::new(Box::new(Box::new((***pivot).saturating_sub(1)))),
            );

            // Sort the higher sub-array
            quicksort(&mut vec, Box::new(Box::new(Box::new(***pivot + 1))), high);
        }
    }

    fn partition(
        vec: &mut Box<Box<Vec<Box<Box<Box<usize>>>>>>,
        low: Box<Box<Box<usize>>>,
        high: Box<Box<Box<usize>>>,
    ) -> Box<Box<Box<usize>>> {
        let pivot = vec[***high].clone();
        let mut i = low.clone();

        // For each element in the sub-array
        for j in ***low..***high {
            // If the item is less than the pivot
            if vec[j] < pivot {
                // Swap the i'th and j'th item
                vec.swap(***i, j);
                // Increment i
                ***i += 1;
            }
        }

        // Swap the highest element and the i'th element
        vec.swap(***i, ***high);

        // Return the pivot point
        i
    }
}

pub mod realloc {
    pub fn quicksort(
        vec: &mut Box<Box<Vec<Box<Box<Box<usize>>>>>>,
        low: Box<Box<Box<usize>>>,
        high: Box<Box<Box<usize>>>,
    ) {
        if low < high {
            // Choose a pivot
            let mut v = vec.clone();
            let pivot = partition(&mut v, low.clone(), high.clone());

            // Sort the lower sub-array
            let mut v_low = v.clone();
            quicksort(
                &mut v_low,
                low.clone(),
                Box::new(Box::new(Box::new((***pivot).saturating_sub(1)))),
            );

            // Sort the higher sub-array
            let mut v_high = v_low.clone();
            quicksort(
                &mut v_high,
                Box::new(Box::new(Box::new(***pivot + 1))),
                high,
            );

            *vec = v_high;
        }
    }

    fn partition(
        vec: &mut Box<Box<Vec<Box<Box<Box<usize>>>>>>,
        low: Box<Box<Box<usize>>>,
        high: Box<Box<Box<usize>>>,
    ) -> Box<Box<Box<usize>>> {
        let mut v = vec.clone();
        let pivot = v.clone()[***high].clone();
        let mut i = low.clone();

        // For each element in the sub-array
        for j in ***low..***high {
            // If the item is less than the pivot
            if v[j] < pivot {
                // Swap the i'th and j'th item
                v.swap(***i, j);
                // Increment i
                ***i += 1;
            }
        }

        // Swap the highest element and the i'th element
        v.swap(***i, ***high);

        *vec = v;

        // Return the pivot point
        i
    }
}

pub mod lockful {
    use std::{
        sync::{Arc, Mutex},
        thread::{self, JoinHandle},
    };

    pub fn quicksort(
        vec: Arc<Mutex<Box<Box<Vec<Box<Box<Box<Arc<Mutex<usize>>>>>>>>>>,
        low: Box<Box<Box<Arc<Mutex<usize>>>>>,
        high: Box<Box<Box<Arc<Mutex<usize>>>>>,
    ) -> JoinHandle<()> {
        thread::spawn(move || {
            if *low.lock().unwrap() < *high.lock().unwrap() {
                // Choose a pivot
                let v = vec.clone();
                let pivot = partition(v, low.clone(), high.clone()).join().unwrap();

                // Sort the lower sub-array
                quicksort(
                    vec.clone(),
                    low.clone(),
                    Box::new(Box::new(Box::new(Arc::new(Mutex::new(
                        (*pivot).lock().unwrap().saturating_sub(1),
                    ))))),
                )
                .join()
                .unwrap();

                // Sort the higher sub-array
                quicksort(
                    vec.clone(),
                    Box::new(Box::new(Box::new(Arc::new(Mutex::new(
                        *pivot.lock().unwrap() + 1,
                    ))))),
                    high,
                )
                .join()
                .unwrap();
            }
        })
    }

    fn partition(
        vec: Arc<Mutex<Box<Box<Vec<Box<Box<Box<Arc<Mutex<usize>>>>>>>>>>,
        low: Box<Box<Box<Arc<Mutex<usize>>>>>,
        high: Box<Box<Box<Arc<Mutex<usize>>>>>,
    ) -> JoinHandle<Box<Box<Box<Arc<Mutex<usize>>>>>> {
        thread::spawn(move || {
            let pivot = vec.lock().unwrap()[*high.lock().unwrap()].clone();
            let i = low.clone();

            // For each element in the sub-array
            for j in *low.lock().unwrap()..*high.lock().unwrap() {
                // If the item is less than the pivot
                if *vec.lock().unwrap()[j].lock().unwrap() < *pivot.lock().unwrap() {
                    // Swap the i'th and j'th item
                    vec.lock().unwrap().swap(*i.lock().unwrap(), j);
                    // Increment i
                    *i.lock().unwrap() += 1;
                }
            }

            // Swap the highest element and the i'th element
            vec.lock()
                .unwrap()
                .swap(*i.lock().unwrap(), *high.lock().unwrap());

            // Return the pivot point
            i
        })
    }
}

pub mod threadpool {
    use std::sync::{mpsc, Arc, Mutex};
    use threadpool::ThreadPool;

    pub fn quicksort(
        vec: Arc<Mutex<Box<Box<Vec<Box<Box<Box<Arc<Mutex<usize>>>>>>>>>>,
        low: Box<Box<Box<Arc<Mutex<usize>>>>>,
        high: Box<Box<Box<Arc<Mutex<usize>>>>>,
        pool: ThreadPool,
    ) {
        let p = pool.clone();
        pool.execute(move || {
            if *low.lock().unwrap() < *high.lock().unwrap() {
                // Choose a pivot
                let v = vec.clone();
                let pivot = partition(v, low.clone(), high.clone(), p.clone());

                // Sort the lower sub-array
                quicksort(
                    vec.clone(),
                    low.clone(),
                    Box::new(Box::new(Box::new(Arc::new(Mutex::new(
                        (*pivot).lock().unwrap().saturating_sub(1),
                    ))))),
                    p.clone(),
                );

                // Sort the higher sub-array
                quicksort(
                    vec.clone(),
                    Box::new(Box::new(Box::new(Arc::new(Mutex::new(
                        *pivot.lock().unwrap() + 1,
                    ))))),
                    high,
                    p.clone(),
                );
            }
        });
    }

    fn partition(
        vec: Arc<Mutex<Box<Box<Vec<Box<Box<Box<Arc<Mutex<usize>>>>>>>>>>,
        low: Box<Box<Box<Arc<Mutex<usize>>>>>,
        high: Box<Box<Box<Arc<Mutex<usize>>>>>,
        pool: ThreadPool,
    ) -> Box<Box<Box<Arc<Mutex<usize>>>>> {
        let (send, recv) = mpsc::channel();

        pool.execute(move || {
            let pivot = vec.lock().unwrap()[*high.lock().unwrap()].clone();
            let i = low.clone();

            // For each element in the sub-array
            for j in *low.lock().unwrap()..*high.lock().unwrap() {
                // If the item is less than the pivot
                if *vec.lock().unwrap()[j].lock().unwrap() < *pivot.lock().unwrap() {
                    // Swap the i'th and j'th item
                    vec.lock().unwrap().swap(*i.lock().unwrap(), j);
                    // Increment i
                    *i.lock().unwrap() += 1;
                }
            }

            // Swap the highest element and the i'th element
            vec.lock()
                .unwrap()
                .swap(*i.lock().unwrap(), *high.lock().unwrap());

            // Return the pivot point
            send.send(i).unwrap();
        });

        recv.recv().unwrap()
    }
}

pub mod locked_no_threads {
    use std::sync::{Arc, Mutex};

    pub fn quicksort(
        vec: Arc<Mutex<Box<Box<Vec<Box<Box<Box<Arc<Mutex<usize>>>>>>>>>>,
        low: Box<Box<Box<Arc<Mutex<usize>>>>>,
        high: Box<Box<Box<Arc<Mutex<usize>>>>>,
    ) {
        if *low.lock().unwrap() < *high.lock().unwrap() {
            // Choose a pivot
            let v = vec.clone();
            let pivot = partition(v, low.clone(), high.clone());

            // Sort the lower sub-array
            quicksort(
                vec.clone(),
                low.clone(),
                Box::new(Box::new(Box::new(Arc::new(Mutex::new(
                    (*pivot).lock().unwrap().saturating_sub(1),
                ))))),
            );

            // Sort the higher sub-array
            quicksort(
                vec.clone(),
                Box::new(Box::new(Box::new(Arc::new(Mutex::new(
                    *pivot.lock().unwrap() + 1,
                ))))),
                high,
            );
        }
    }

    fn partition(
        vec: Arc<Mutex<Box<Box<Vec<Box<Box<Box<Arc<Mutex<usize>>>>>>>>>>,
        low: Box<Box<Box<Arc<Mutex<usize>>>>>,
        high: Box<Box<Box<Arc<Mutex<usize>>>>>,
    ) -> Box<Box<Box<Arc<Mutex<usize>>>>> {
        let pivot = vec.lock().unwrap()[*high.lock().unwrap()].clone();
        let i = low.clone();

        // For each element in the sub-array
        for j in *low.lock().unwrap()..*high.lock().unwrap() {
            // If the item is less than the pivot
            if *vec.lock().unwrap()[j].lock().unwrap() < *pivot.lock().unwrap() {
                // Swap the i'th and j'th item
                vec.lock().unwrap().swap(*i.lock().unwrap(), j);
                // Increment i
                *i.lock().unwrap() += 1;
            }
        }

        // Swap the highest element and the i'th element
        vec.lock()
            .unwrap()
            .swap(*i.lock().unwrap(), *high.lock().unwrap());

        i
    }
}
