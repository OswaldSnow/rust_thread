use std::thread;
use std::time::Duration;

fn main() {
    /*
    多线程
     */

    let mut num = 9;
    let mut women_name = String::from("Women");

    // 将使用到的变量的所有权移动到线程中
    let thread1 = thread::spawn(move || {
        for i in 1..=3 {
            println!("thread1 thread loop cur-value is {}", i);
        }
        women_name.push_str("Karima!");
        num += 1;
        (women_name, num)
    });

    // 非 main 线程的子线程不会在父线程执行完成后结束，只会在线程本身代码执行完成后结束
    // thread2 中的子线程不会受 thread2 线程是否结束影响
    let thread2 = thread::spawn(move || {
        for i in 1..=3 {
            println!("thread2 thread loop cur-value is {}", i);
        }

        thread::spawn(|| loop {
            println!("this is child's child thread!")
        })
    });

    let _thread2_result = thread2.join().unwrap();

    /*
    虽然以下循环在两个线程join中间
    但是要等到两个线程结束之后才会执行
     */
    for i in 1..=3 {
        println!("main thread loop cur-value is {}", i);
    }

    let thread1_result = thread1.join().unwrap();

    println!("yes! all thread is done! num is {:?}", thread1_result);

    // 基本类型 i32 num 被复制了一份到线程中，主线程 main 中的 num 不受影响
    println!("main current num values is {num}");

    // 休眠1s，查看 thread2 子线程执行情况
    println!("开始休眠");
    thread::sleep(Duration::from_millis(1));
}
