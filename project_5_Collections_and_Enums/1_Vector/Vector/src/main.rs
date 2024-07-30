// 此文件展示了 Rust 中 Vector 的基本用法
fn main() {
    // 创建一个空的可存储 i32 类型的 Vector
    let mut v: Vec<i32> = Vec::new();

    // 向 Vector 中添加元素
    v.push(1);
    v.push(2);
    v.push(3);

    // 打印 Vector 的内容
    println!("{:?}", v);

    // 另一种创建 Vector 并初始化的方式
    let v2 = vec![1, 2, 3];

    // 遍历 Vector 并打印每个元素
    for i in &v2 {
        println!("{}", i);
    }
}
