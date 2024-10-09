use std::thread;
fn main() {
    /*
    Send 特征 和 Sync 特征
    Send 和 Sync 是多线程安全使用值的关键
    1、实现 Send 的类型可以在多线程中安全的传递所有权
    2、实现 Sync 的类型可以在多线程中安全的共享

    一个类型如果要在多个线程中共享，那么此类型就需要实现 Sync
    共享那就需要使用引用，那么这个类型的引用类型 &T 就需要实现 Send

    引用实现 Send 保证引用能在多个线程传输
    本身实现 Sync 保证多个线程能共享数据

    通常不需要手动实现 Send 和 Sync
     */

    /*
    为裸指针实现 Send 和 Sync
     */

    // p 是一个指针 指向内存地址为 8 的地址，这里的 8 并不是一个合法的
    let p = 8 as *mut u8;

    // 编译错误：the trait `Send` is not implemented for `*mut u8`, which is required by
    // thread::spawn(move || {
    //     println!("p val is {:?}",p);
    // });

    let my_box = MyBox(p);
    let handle = thread::spawn(move || {
        println!("my_box is {:?}", my_box);
        // 使用 my_box.0 编译器会提示错误
    });

    handle.join().unwrap();
}

#[derive(Debug)]
#[allow(unused)]
struct MyBox(*mut u8);

unsafe impl Send for MyBox {}
unsafe impl Sync for MyBox {}
