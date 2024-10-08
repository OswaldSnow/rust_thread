use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    /*
    读写锁 RwLock
    1、同一时间只能存在一个 写，和多个 读，当有线程获取了写锁后，其他尝试获取读锁的线程阻塞 等待 写锁释放
    2、不能同时存在多个 写锁
    3、使用 read、write 和 try_read、try_write
     */

    let rwlock = Arc::new(RwLock::new(String::from("hello RwLock")));

    let rwlock_clone1 = rwlock.clone();
    let handle1 = thread::spawn(move || {
        println!("in thread 1 rwlock: {}", rwlock_clone1.read().unwrap());
    });

    let rwlock_clone2 = rwlock.clone();
    let handle2 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        let mut write_lock = rwlock_clone2.write().unwrap();
        write_lock.push_str(" how about you?");
        println!("in thread 2 rwlock: {}", write_lock);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("main thread will end");
}
