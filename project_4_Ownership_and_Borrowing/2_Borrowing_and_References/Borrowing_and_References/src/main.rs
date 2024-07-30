fn main() {
    // 不可变引用
    {
        let s = String::from("hello");
        let len = calculate_length(&s); // 不可变引用
        println!("The length of '{}' is {}.", s, len);
    }

    // 可变引用
    {
        let mut s2 = String::from("hello");
        change(&mut s2); // 可变引用
        println!("Changed string: {}", s2);
    }

    // 生命周期标注
    {
        let string1 = String::from("long string");
        let string2 = String::from("short");
        let result = longest(&string1, &string2);
        println!("The longest string is {}", result);
    }

    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    fn change(s: &mut String) {
        s.push_str(", world");
    }

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
