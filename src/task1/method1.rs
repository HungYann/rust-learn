#[derive(Debug)]
enum MyNumber {
    Int(i32),
    Float(f64),
    String(String),
}

impl MyNumber {
    fn add(&self, other: &Self) -> Self {
        match (self, other) {
            (MyNumber::Int(a), MyNumber::Int(b)) => MyNumber::Int(a + b),
            (MyNumber::Float(a), MyNumber::Float(b)) => MyNumber::Float(a + b),
            (MyNumber::String(a), MyNumber::String(b)) => MyNumber::String(format!("{}{}", a, b)),
            _ => unimplemented!("Cannot add these types"),
        }
    }
}

fn main() {
    let numbers = vec![
        MyNumber::Int(5),
        MyNumber::Float(3.14),
        MyNumber::String(String::from("Hello, ")),
    ];

    for number in numbers {
        let result = number.add(&MyNumber::Int(3));
        println!("Result: {:?}", result);
    }
}