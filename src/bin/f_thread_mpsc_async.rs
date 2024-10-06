use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    /*
    异步消息通道
     */

    let (sen, rec) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("Black"),
            String::from("Green"),
            String::from("Red"),
            String::from("Orange"),
        ];

        for val in vals {
            sen.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // 阻塞接收消息
    // if let Ok(val) = rec.recv(){
    //     println!("{}", val);
    // }

    // 不阻塞接收消息
    // println!("{:?}", rec.try_recv());
    // thread::sleep(Duration::from_millis(2000));
    // println!("{:?}", rec.try_recv());
    // thread::sleep(Duration::from_millis(3000));
    // println!("{:?}", rec.try_recv());

    // 循环接收消息
    // 阻塞 等待接收 接收到消息 打印 发送者关闭时结束循环
    for received in rec {
        println!("{}", received);
    }

    /*
    通道消息接收的顺序满足 FIFO（先进先出）原则
     */
}
