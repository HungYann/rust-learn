
## 笔记一

元组结构体：所谓元组结构体，也就是元组和结构体的结合体

```rust
struct Color(i32, i32);

fn main() {
    let _black = Color(0,  0);
}
```

什么是所有权？


一句话解释：谁（获得着）控制这个数据的生死权利

在Rust中，每一个值都有一个决定其生命周期的唯一所有者（owner）

所有权的存在原因？

- 跟踪代码的哪些部分正在使用heap的哪些数据
- 最小化heap上的重复数据
- 清理heap上未使用的数据以免空间不足

## 笔记二

`struct`结构体

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u32,
}

fn main() {
    let active = true;
    let username = String::from("someusername123");
    let email = String::from("someuser@example123");
    let user1 = User {
        active,
        username,
        email,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

`enum`枚举


`if let`
```rust
fn main() {
    let mut optional = Some(0);

    if let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit");
            optional = None;
        } else {
            println!("t");
        }
    }
}
```


`Vec`与HashMap

```rust
fn foo1(s: &str) {

}

fn foo2(s: &[u32]) {

}

fn main() {
    let s = String::from("aaa");
    foo1(&s);

    foo1("aaaabbb");

    let v: Vec<u32> = vec![1, 2, 3, 4, 5];
    foo2(&v);

    foo2(&[1, 2, 3, 4, 5]);
}
```





