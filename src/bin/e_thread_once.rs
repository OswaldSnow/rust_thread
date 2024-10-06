use std::sync::Once;
use std::thread;

static mut VAL: usize = 0;
static INIT: Once = Once::new();

fn main() {
    /*
    某个函数在多线程环境下只被调用一次
     */

    let handle1 = thread::spawn(move || {
        INIT.call_once(|| {
            unsafe {
                VAL += 1;
            }
            println!("handle1 VAL += 1");
        });

        println!("handle1 done!");
    });

    let handle2 = thread::spawn(|| {
        INIT.call_once(|| {
            unsafe {
                VAL += 2;
            }
            println!("handle2 VAL += 2");
        });

        println!("handle2 done!");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", unsafe { VAL });
}
