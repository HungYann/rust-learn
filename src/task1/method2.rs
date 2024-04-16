// 定义一个 trait,包含三种类型的共同方法
trait MyTrait {
    fn do_something(&self);
}

// 为三种类型分别实现 MyTrait
impl MyTrait for TypeA {
    fn do_something(&self) {
        println!("TypeA is doing something");
    }
}

impl MyTrait for TypeB {
    fn do_something(&self) {
        println!("TypeB is doing something");
    }
}

impl MyTrait for TypeC {
    fn do_something(&self) {
        println!("TypeC is doing something");
    }
}

fn main() {
    // 创建三个不同类型的实例,并放入 Vec 中
    let my_types: Vec<Box<dyn MyTrait>> = vec![
        Box::new(TypeA),
        Box::new(TypeB),
        Box::new(TypeC),
    ];

    // 遍历 Vec,并调用相应类型的方法
    for my_type in my_types {
        my_type.do_something();
    }
}