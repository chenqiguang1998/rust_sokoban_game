fn main() {
    // 所有权规则
    {
        let s = String::from("hello");
        // s 是该字符串的所有者
        println!("{}", s);
        // s 离开作用域，字符串被释放
    }

    // 所有权的转移
    {
        let s1 = String::from("hello");
        let s2 = s1; // s1 的所有权被转移到 s2
        // println!("{}", s1); // 这行代码会报错，因为 s1 不再拥有该值
        println!("{}", s2);
    }

    // 函数与所有权
    {
        let s = String::from("hello");
        take_ownership(s); // s 的所有权被转移到函数
        // println!("{}", s); // 这行代码会报错，因为 s 的所有权已经被转移

        let x = 5;
        makes_copy(x); // x 是一个基本数据类型，实现了 Copy trait，所以不会转移所有权
        println!("{}", x); // 这行代码可以正常运行
    }

    fn take_ownership(some_string: String) {
        println!("{}", some_string);
    }

    fn makes_copy(some_integer: i32) {
        println!("{}", some_integer);
    }
}
