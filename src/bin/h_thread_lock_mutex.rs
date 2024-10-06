use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use std::{
    sync::{Mutex, MutexGuard},
    thread,
};

// use lazy_static::lazy_static;
// lazy_static! {
//     static ref MUTEX1: Mutex<i64> = Mutex::new(1);
//     static ref MUTEX2: Mutex<i64> = Mutex::new(2);
// }

fn main() {
    /*
    有概率会造成多线程死锁的代码
     */

    let mutex1 = Arc::new(Mutex::new(1));
    let mutex2 = Arc::new(Mutex::new(2));

    // 存放子线程的句柄
    let mut children = vec![];
    for i_thread in 0..2 {
        let mutex1_clone = mutex1.clone();
        let mutex2_clone = mutex2.clone();
        children.push(thread::spawn(move || {
            // 线程1
            if i_thread % 2 == 0 {
                // 锁住MUTEX1
                let _guard: MutexGuard<i64> = mutex1_clone.lock().unwrap();

                println!("线程 {} 锁住了MUTEX1，接着准备去锁MUTEX2 !", i_thread);

                // 当前线程睡眠一小会儿，等待线程2锁住MUTEX2
                sleep(Duration::from_millis(10));

                // 去锁MUTEX2
                // let guard = MUTEX2.lock().unwrap();

                // 使用 try_lock 解决死锁
                let guard = mutex2_clone.try_lock();
                match guard {
                    Ok(t) => {
                        println!("thread0 try_lock MUTEX2 failed success {:?}", t)
                    }
                    Err(e) => {
                        println!("thread0 try_lock MUTEX2 failed {:?}", e)
                    }
                }

            // 线程2
            } else {
                // 锁住MUTEX2
                let _guard = mutex2_clone.lock().unwrap();

                println!("线程 {} 锁住了MUTEX2, 准备去锁MUTEX1", i_thread);

                // let _guard = MUTEX1.lock().unwrap();

                // 使用 try_lock 解决死锁
                let guard = mutex1_clone.try_lock();
                match guard {
                    Ok(t) => {
                        println!("thread1 try_lock MUTEX1 success {:?}", t);
                    }
                    Err(e) => {
                        println!("thread1 try_lock MUTEX1 failed {:?}", e);
                    }
                }
            }
        }));
    }

    // 等子线程完成
    for child in children {
        let _ = child.join();
    }

    println!("死锁没有发生");

    /*
    以上代码是否会造成死锁与两个线程的执行顺序有关
    如果一个线程在另一个线程开始前就执行结束了 那么不会造成死锁

    但是：

    当 0线程已经锁住了 MUTEX1
    且 1线程已经锁出了 MUTEX2

    接下来

    0线程继续尝试锁住 MUTEX2 但是由于 MUTEX2 被1线程锁住 0线程阻塞等待
    1线程继续尝试锁住 MUTEX1 但是由于 MUTEX1 被0线程锁住 1线程阻塞等待

    两个线程都阻塞等待 而每个线程最开始锁住的数据（0线程-MUTEX1 1线程-MUTEX2）
    又必须在当前线程执行完成结束之后才能释放

    两个线程都在等待

    综上 造成了程序死锁
     */

    /*
    .lock 方法是阻塞的
    .try_lock 方法是非阻塞的
     */
}
