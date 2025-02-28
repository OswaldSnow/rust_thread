use std::sync::{Arc, Condvar, Mutex};
use std::thread::{sleep, spawn};
use std::time::Duration;

fn main() {
    /*
    梳理清楚以下代码执行过程
    .join 方法一般用于阻塞主线程 等待相关线程执行结束
    分析下方代码的关键在于：cond.wait() 会释放相关数据的互斥锁并且阻塞当前线程直到收到通知
     */
    let flag = Arc::new(Mutex::new(false));
    let cond = Arc::new(Condvar::new());
    let cflag = flag.clone();
    let ccond = cond.clone();

    let hdl = spawn(move || {
        println!("thread is begin");
        let mut lock = cflag.lock().unwrap();
        let mut counter = 0;

        while counter < 3 {
            while !*lock {
                // wait方法会接收一个MutexGuard<'a, T>
                // 且它会自动地暂时释放这个锁，使其他线程可以拿到锁并进行数据更新
                // 同时当前线程在此处会被阻塞，直到被其他地方notify后
                // 它会将原本的MutexGuard<'a, T>还给我们，即重新获取到了锁，同时唤醒了此线程
                lock = ccond.wait(lock).unwrap();
            }

            *lock = false;

            counter += 1;
            println!("inner counter: {}", counter);
        }
    });

    let mut counter = 0;
    loop {
        sleep(Duration::from_millis(1000));
        *flag.lock().unwrap() = true;
        counter += 1;
        if counter > 3 {
            break;
        }
        println!("outside counter: {}", counter);
        cond.notify_one();
    }
    hdl.join().unwrap();
    println!("{:?}", flag);
}
