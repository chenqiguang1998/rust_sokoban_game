// 引入所需的模块
use std::thread;  // 用于创建和管理线程
use std::time::Duration;  // 用于控制线程的睡眠时间

// 主函数
fn main() {
    // 使用 thread::spawn 创建一个新的线程，并将一个闭包作为线程的执行体
    let handle = thread::spawn(|| {
        // 在新线程中执行的循环
        for i in 1..10 {
            // 打印新线程中的信息
            println!("Hello from the spawned thread: {}", i);
            // 让新线程睡眠 1 毫秒，模拟耗时操作
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 在主线程中执行的循环
    for i in 1..5 {
        // 打印主线程中的信息
        println!("Hello from the main thread: {}", i);
        // 让主线程睡眠 1 毫秒，模拟耗时操作
        thread::sleep(Duration::from_millis(1));
    }

    // 等待新线程执行完毕
    handle.join().unwrap();
}
