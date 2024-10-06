use std::cell::Cell;
use std::thread;

thread_local! {
    static COUNTER: Cell<usize> = Cell::new(0);
}

fn main() {
    /*
    thread local
    线程本地变量
     */

    let handles: Vec<_> = (0..5)
        .map(|i| {
            thread::spawn(move || {
                // 每个线程都会拿到 thread_local 数据的初始值
                // 一下打印数据的 count 都为 1
                COUNTER.with(|counter| {
                    let count = counter.get();
                    counter.set(count + 1);
                    println!("current thread {i} counter value: {}", counter.get());
                })
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
