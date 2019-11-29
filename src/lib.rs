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
        sync::{mpsc, Arc, Mutex},
        thread,
    };

    pub fn quicksort(
        vec: Arc<Mutex<Box<Box<Vec<Box<Box<Box<Arc<Mutex<usize>>>>>>>>>>,
        low: Box<Box<Box<Arc<Mutex<usize>>>>>,
        high: Box<Box<Box<Arc<Mutex<usize>>>>>,
    ) {
        // Create a channel so we know when the algorithm is done
        let (send, recv) = mpsc::channel::<()>();

        // Spawn the thread that will do all the work
        thread::spawn(move || {
            if *low.lock().unwrap() < *high.lock().unwrap() {
                // Choose a pivot
                let pivot = partition(vec.clone(), low.clone(), high.clone());

                // Sort the lower sub-array
                quicksort(
                    vec.clone(),
                    low.clone(),
                    Box::new(Box::new(Box::new(Arc::new(Mutex::new(
                        pivot.saturating_sub(1),
                    ))))),
                );

                // Sort the higher sub-array
                quicksort(
                    vec.clone(),
                    Box::new(Box::new(Box::new(Arc::new(Mutex::new(pivot + 1))))),
                    high,
                );
            }

            // Notify the main thread that the calculation is finished
            send.send(()).unwrap();
        });

        // Block the thread until the calculation is finished
        recv.recv().unwrap()
    }

    fn partition(
        vec: Arc<Mutex<Box<Box<Vec<Box<Box<Box<Arc<Mutex<usize>>>>>>>>>>,
        low: Box<Box<Box<Arc<Mutex<usize>>>>>,
        high: Box<Box<Box<Arc<Mutex<usize>>>>>,
    ) -> usize {
        // Create a channel so we can get the pivot point
        let (send, recv) = mpsc::channel::<usize>();

        // Spawn the thread that will do all the calculation
        thread::spawn(move || {
            let mut i = *low.lock().unwrap();

            // For each element in the sub-array
            let (l, h): (usize, usize) = (*low.lock().unwrap(), *high.lock().unwrap());
            for j in l..h {
                // If the item is less than the pivot
                let piv: usize = *vec.lock().unwrap()[*high.lock().unwrap()].lock().unwrap();
                let left: usize = *vec.lock().unwrap()[j].lock().unwrap();
                if left < piv {
                    // Swap the i'th and j'th item
                    (*vec.lock().unwrap()).swap(i, j);

                    // Increment i
                    i += 1;
                }
            }

            // Swap the highest element and the i'th element
            let temp_high = *high.lock().unwrap();
            vec.lock().unwrap().swap(i, temp_high);

            // Return the pivot point
            send.send(i).unwrap();
        });

        // Get the pivot point and return it
        recv.recv().unwrap()
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
        // Clone the threadpool (Only clones pointers) so we can move it into the threadpool closure
        let p = pool.clone();

        // Create a channel so we know when the algorithm is done
        let (send, recv) = mpsc::channel::<()>();

        // Execute the function in the threadpool
        pool.execute(move || {
            if *low.lock().unwrap() < *high.lock().unwrap() {
                // Choose a pivot
                let pivot = partition(vec.clone(), low.clone(), high.clone(), p.clone());

                // Sort the lower sub-array
                quicksort(
                    vec.clone(),
                    low.clone(),
                    Box::new(Box::new(Box::new(Arc::new(Mutex::new(
                        pivot.saturating_sub(1),
                    ))))),
                    p.clone(),
                );

                // Sort the higher sub-array
                quicksort(
                    vec.clone(),
                    Box::new(Box::new(Box::new(Arc::new(Mutex::new(pivot + 1))))),
                    high,
                    p.clone(),
                );
            }

            // The calculation is finished, so tell the main thread
            send.send(()).unwrap();
        });

        // Blocks the thread until the calculation is finished
        recv.recv().unwrap()
    }

    fn partition(
        vec: Arc<Mutex<Box<Box<Vec<Box<Box<Box<Arc<Mutex<usize>>>>>>>>>>,
        low: Box<Box<Box<Arc<Mutex<usize>>>>>,
        high: Box<Box<Box<Arc<Mutex<usize>>>>>,
        pool: ThreadPool,
    ) -> usize {
        // Create the sender and receiver for communicating with the threadpool
        let (send, recv) = mpsc::channel();

        // Execute the function in the threadpool
        pool.execute(move || {
            let mut i = *low.lock().unwrap();

            // For each element in the sub-array
            let (l, h) = (*low.lock().unwrap(), *high.lock().unwrap());
            for j in l..h {
                let pivot_value = *vec.lock().unwrap()[*high.lock().unwrap()].lock().unwrap();
                let j_value = *vec.lock().unwrap()[j].lock().unwrap();

                // If the item is less than the pivot
                if j_value < pivot_value {
                    // Swap the i'th and j'th item
                    (*vec.lock().unwrap()).swap(i, j);

                    // Increment i
                    i += 1;
                }
            }

            // Swap the highest element and the i'th element
            vec.lock().unwrap().swap(i, *high.lock().unwrap());

            // Send the pivot point
            send.send(i).unwrap();
        });

        // Receive and return the pivot point
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
            let pivot = partition(vec.clone(), low.clone(), high.clone());

            // Sort the lower sub-array
            quicksort(
                vec.clone(),
                low.clone(),
                Box::new(Box::new(Box::new(Arc::new(Mutex::new(
                    pivot.saturating_sub(1),
                ))))),
            );

            // Sort the higher sub-array
            quicksort(
                vec.clone(),
                Box::new(Box::new(Box::new(Arc::new(Mutex::new(pivot + 1))))),
                high,
            );
        }
    }

    fn partition(
        vec: Arc<Mutex<Box<Box<Vec<Box<Box<Box<Arc<Mutex<usize>>>>>>>>>>,
        low: Box<Box<Box<Arc<Mutex<usize>>>>>,
        high: Box<Box<Box<Arc<Mutex<usize>>>>>,
    ) -> usize {
        let mut i = *low.lock().unwrap();

        // For each element in the sub-array
        let (l, h): (usize, usize) = (*low.lock().unwrap(), *high.lock().unwrap());
        for j in l..h {
            let piv: usize = *vec.lock().unwrap()[*high.lock().unwrap()].lock().unwrap();
            let left: usize = *vec.lock().unwrap()[j].lock().unwrap();

            // If the item is less than the pivot
            if left < piv {
                // Swap the i'th and j'th item
                (*vec.lock().unwrap()).swap(i, j);

                // Increment i
                i += 1;
            }
        }

        // Swap the highest element and the i'th element
        let temp_high = *high.lock().unwrap();
        vec.lock().unwrap().swap(i, temp_high);

        // Return the pivot point
        i
    }
}
