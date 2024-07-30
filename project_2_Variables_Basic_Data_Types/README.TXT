# Rust 中的变量、常量与基本数据类型

在这部分内容中，我们将深入探讨 Rust 中变量、常量和基本数据类型的相关知识，并通过示例来帮助您理解如何在推箱子游戏中应用这些概念。

## 变量与常量

### 变量定义
在 Rust 中，使用 `let` 关键字定义变量，默认情况下变量是不可变的。若要定义可变变量，则使用 `mut` 关键字。

```rust
let x = 5; // 不可变变量
let mut y = 10; // 可变变量
```

与其他主流语言（如 Java、C++ 等）相比，Rust 对变量的可变性有更严格的控制，强调了不可变变量的默认使用，以提高程序的安全性和可维护性。

### 常量定义
常量使用 `const` 关键字定义，并在声明时必须指定类型，且在整个程序生命周期内保持不变。

```rust
const MAX_SCORE: u32 = 100;
```

在许多语言中，常量的定义和使用方式类似，但 Rust 对常量的类型要求更加明确。

### 变量的作用域和生命周期
变量的作用域从声明位置开始，到当前代码块结束。Rust 通过所有权和借用机制来管理变量的生命周期。

```rust
{
    let z = 20; // 变量 z 的作用域在这个代码块内
    println!("z: {}", z);
}
// 此处无法访问 z，因为已超出作用域
```

这与主流语言中的作用域规则有一定的相似性，但 Rust 的所有权机制为内存安全提供了更强大的保障。

## 基本数据类型

### 整数
Rust 支持多种整数类型，如 `i32`（32 位有符号整数）。

```rust
let a: i32 = 10;
```

### 浮点数
例如 `f64`（64 位浮点数）。

```rust
let b: f64 = 20.5;
```

### 布尔值
`bool` 类型用于表示真或假。

```rust
let c: bool = true;
```

### 字符
使用 `char` 类型表示单个字符。

```rust
let d: char = 'R';
```

### 数组与切片
数组是固定大小、相同类型元素的集合，切片是对数组或向量一部分的引用。

```rust
let arr: [i32; 3] = [1, 2, 3]; // 数组
let slice: &[i32] = &arr[1..3]; // 切片
```

### 元组
元组可以包含不同类型的多个值。

```rust
let tuple: (i32, f64, char) = (10, 20.5, 'R');
```

### 结构体
具有命名字段的数据类型。

```rust
struct Player {
    name: String,
    score: u32,
}
```

通过学习这些变量、常量和基本数据类型，您将能够为推箱子游戏创建和管理各种数据。

## 示例代码：简单的游戏状态

我们创建了一个简单的 `GameState` 结构体来表示游戏状态，并进行了初始化。

```rust
struct GameState {
    player_position: (i32, i32),
    box_position: (i32, i32),
    target_position: (i32, i32),
}

impl GameState {
    fn new() -> Self {
        GameState {
            player_position: (0, 0),
            box_position: (1, 1),
            target_position: (2, 2),
        }
    }
}

fn main() {
    let game_state = GameState::new();
    println!(
        "Player Position: {:?}, Box Position: {:?}, Target Position: {:?}",
        game_state.player_position, game_state.box_position, game_state.target_position
    );
}
```

通过这部分的学习，您将掌握 Rust 中的变量、常量和基本数据类型，并能够创建和初始化简单的游戏状态，为实现推箱子游戏的核心功能打下基础。接下来，我们将介绍函数与控制流，继续完善游戏的功能。

