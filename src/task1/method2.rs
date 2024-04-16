// 定义三个不同的类型
struct TypeA;
struct TypeB;
struct TypeC;

// 定义一个枚举,包裹三种不同的类型
enum MyTypes {
    A(TypeA),
    B(TypeB),
    C(TypeC),
}

// 为每个类型实现一个方法
impl TypeA {
    fn do_something_a(&self) {
        println!("TypeA is doing something");
    }
}

impl TypeB {
    fn do_something_b(&self) {
        println!("TypeB is doing something");
    }
}

impl TypeC {
    fn do_something_c(&self) {
        println!("TypeC is doing something");
    }
}

fn main() {
    // 创建三个不同类型的实例,并放入 Vec 中
    let my_types = vec![
        MyTypes::A(TypeA),
        MyTypes::B(TypeB),
        MyTypes::C(TypeC),
    ];

    // 遍历 Vec,并调用相应类型的方法
    for my_type in my_types {
        match my_type {
            MyTypes::A(a) => a.do_something_a(),
            MyTypes::B(b) => b.do_something_b(),
            MyTypes::C(c) => c.do_something_c(),
        }
    }
}