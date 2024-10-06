use std::sync::mpsc::sync_channel;
use std::thread;
use std::time::Duration;

fn main() {
    /*
    同步消息通道
     */

    // sync_channel(N) 其中的 N 参数表示同步消息的缓存数量
    // 当消息缓存未达到 N 条时 可以 不阻塞的发送消息
    // 当缓存队列满时 发送消息将会阻塞 直到有消息被接收
    let (send, receive) = sync_channel(0);

    thread::spawn(move || {
        println!("send before");
        // 同步消息通道发送消息时将会阻塞 直到消息被接收
        send.send("this is sync message").unwrap();
        println!("send after");
    });

    thread::sleep(Duration::from_millis(1000));

    println!("receive message is {:?}", receive.recv());

    /*
    当 【所有的发送者被drop】 或 【所有的接收者被 drop】通道将会自动关闭

    当为了在多个线程中使用同一个发送者而使用 Arc 进行 clone 时
    当 线程 drop 时，原始的 消息通道 并未被 drop 只是 Arc 引用被 drop
    所以可能会发生 【消息通道一直未关闭】的情况 望周知
     */
}
