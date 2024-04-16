// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// struct Point {
//     x: i32,
//     y: i32,
// }
//
// impl std::ops::Add for Point {
//     type Output = Self;
//
//     fn add(self, other: Self) -> Self {
//         Point {
//             x: self.x + other.x,
//             y: self.y + other.y,
//         }
//     }
// }
//
// trait Addable {
//     fn add(&self, other: &Self) -> Self;
// }
//
// impl Addable for Point {
//     fn add(&self, other: &Self) -> Self {
//         Point {
//             x: self.x + other.x,
//             y: self.y + other.y,
//         }
//     }
// }
//
// fn main() {
//     let p1 = Point { x: 1, y: 2 };
//     let p2 = Point { x: 3, y: 4 };
//
//     // 使用 + 运算符直接相加
//     let p3 = p1 + p2;
//     println!("p1 + p2 = {:?}", p3);
//
//     // 使用 Trait Object 调用 add 方法
//     let addable: &dyn Addable = &p1;
//     let p4 = addable.add(&p2);
//     println!("Using Trait Object: {:?}", p4);
// }