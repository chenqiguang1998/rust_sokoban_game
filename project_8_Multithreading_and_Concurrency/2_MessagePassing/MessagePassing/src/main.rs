// 引入所需的模块
use std::sync::mpsc;  // 用于实现线程间的消息传递
use std::thread;  // 用于创建线程

// 主函数
fn main() {
    // 创建一个消息发送者（tx）和消息接收者（rx）
    let (tx, rx) = mpsc::channel();

    // 使用 thread::spawn 创建一个新的线程
    thread::spawn(move || {
        // 在新线程中创建一个字符串值
        let val = String::from("hello");
        // 向消息发送者发送字符串值
        tx.send(val).unwrap();
    });

    // 从消息接收者接收消息
    let received = rx.recv().unwrap();
    // 打印接收到的消息
    println!("Got: {}", received);
}
