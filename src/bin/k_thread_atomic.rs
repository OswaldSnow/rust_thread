use std::ops::Sub;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread::{self, JoinHandle};
use std::time::Instant;

const N_TIMES: u64 = 10000000;
const N_THREADS: usize = 10;

static R: AtomicU64 = AtomicU64::new(0);

fn add_n_times(n: u64) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..n {
            R.fetch_add(1, Ordering::SeqCst);
        }
    })
}

fn main() {
    /*
    原子操作 Atomic
     */

    let s = Instant::now();
    let mut threads = Vec::with_capacity(N_THREADS);

    for _ in 0..N_THREADS {
        threads.push(add_n_times(N_TIMES));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    // assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));
    // Ordering::xxx 设定内存顺序
    println!("R val is {}", R.load(Ordering::SeqCst));

    println!("{:?}", Instant::now().sub(s));
}
