use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    /*
    互斥锁 Mutex 配合 线程条件 Condvar
     */
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        println!("thread start running");
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true; // 修改共享状态
        cvar.notify_one(); // 通知主线程
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();

    while !*started {
        println!("... before wait");
        started = cvar.wait(started).unwrap(); // 等待通知
    }

    println!("after wait");

    /*
    将 Mutex 和 Condvar 配合可以做到：

    等待和通知机制：
    当一个线程需要等待某个条件（如共享数据的状态）时，它可以使用 Condvar::wait() 方法。
    这会使线程阻塞，直到其他线程调用 notify_one() 或 notify_all() 来唤醒它。

    这种机制允许线程在条件不满足时挂起，而不是忙等待（busy waiting），从而节省 CPU 资源。
    避免不必要的轮询：
    如果只依赖于 Mutex，线程可能会不断尝试获取锁并检查条件，这会导致不必要的 CPU 使用。
    使用 Condvar 可以让线程在条件未满足时进入休眠状态，直到被唤醒。

    复杂的同步场景：
    在一些复杂的场景中，例如生产者-消费者模型，生产者可能需要等待消费者处理完数据后才能继续生产。
    这种情况下，仅用 Mutex 可能无法有效管理状态变化，而 Condvar 则提供了优雅的解决方案。
     */

    /*
    释放锁：当线程调用 wait 时，它会释放持有的互斥锁。
    这是关键步骤，因为如果不释放锁，其他线程将无法获取锁并更新共享状态。

    进入等待状态：当前线程进入阻塞状态，直到条件变量被通知。
    这种设计避免了忙等待（busy waiting），使得 CPU 可以处理其他任务。

    被唤醒后重新获取锁：一旦条件满足并且调用了 notify_one() 或 notify_all()，等待的线程会被唤醒。
    此时，wait 方法会返回，并且该线程会重新获取互斥锁。

    检查条件：在重新获取锁后，线程可以检查共享状态（如 *started），以决定是否继续执行或再次等待。
     */
}
