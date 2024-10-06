use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    /*
    线程屏障 Barrier
     */

    // 使用 Arc 多线程共享
    let barrier6 = Arc::new(Barrier::new(6));
    let mut handles = Vec::with_capacity(6);

    for i in 1..=6 {
        let barrier = barrier6.clone();
        handles.push(thread::spawn(move || {
            println!("this is {} thread println wait", i);
            // 只有当 Barrier 中的 n 个线程都执行到此处后，才能继续执行
            // Barrier 可以保证所有线程都准备好再进行下一步
            barrier.wait();
            println!("this is {} thread println finished", i);
        }))
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
