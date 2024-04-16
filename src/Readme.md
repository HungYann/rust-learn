
## 笔记一

泛型

```rust
fn largest<T: std::cmp::PartialOrd + Copy> (list: &Vec<T>) -> &T {
    let mut largest = &list[0];

    for item in list {
        if *item > *largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![32, 50, 25, 100, 65];
    let result = largest(&number_list);

    println!("The largest number is {}", result);


    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);

    println!("The largest char is {}", result);
}
```


## 笔记二

结构体由一些字段组成。字段是有类型的，这个类型当然适用于泛型。因此，

结构体中，是可以出现泛型的。

```rust
struct Point<T, U> {
    x:T,
    y:U,
}

fn main() {
    let _both_integer = Point{x:5, y:0};
    let _both_float = Point{x:1.0, y:4.0};
}
```

## 笔记三

trait实际是对类型的约束，或者说是代表了一个类（满足条件的）类型。

```rust
trait Animal {
    fn myself(self) -> Self;

    fn eat(&self);

    fn eat_mut(&mut self);
}
```


```rust
use std::fmt::Display;

struct Pair<T> {
    x:T,
    y:T,
}

impl<T> Pair<T> {
    fn new(x:T, y:T) ->Self {
        Self {x, y}
    }
}

impl <T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is ={}", self.x);
        } else {
            println!("The largest member is y={}", self.y);
        }
    }
}

```

## 笔记四

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```


```rust
trait Animal {
    fn talk(&self);
}

struct Cat{}

struct Dog{}

impl Animal for Cat {
    fn talk(&self) {
        println!("meow");
    }
}

impl Animal for Dog {
    fn talk(&self) {
        println!("bark");
    }
}

fn animal_talk<T: Animal>(a: &T) {
    a.talk();
}

fn main() {
    let d = Dog{};
    let c = Cat{};

    animal_talk(&d);
    animal_talk(&c);
}
```
## 笔记五

 ```rust
 trait TraitA {
    fn do_something(&self);
}

struct TypeA;

struct TypeB;

impl TraitA for TypeA {
    fn do_something(&self) {
        println!("TypeA is dothing something");
    }
}

impl TraitA for TypeB {
    fn do_something(&self) {
        println!("TypeB is dothing something");
    }
}

fn main() {
    let type_a = TypeA;

    let type_b = TypeB;

    let a: &dyn TraitA = &type_a;
    let b: &dyn TraitA = &type_b;

    a.do_something();
    b.do_something();
}
 ```

## 笔记六

- TypeT 实现了 TraitA 中定义的所有方法。这意味着 TypeT 满足了 TraitA 的所有要求。

- 当一个函数接受 &dyn TraitA 类型的参数时, 你可以传入一个 &TypeT 类型的值, 因为 TypeT 实现了 TraitA。

- 你可以将 TypeT 作为 TraitA 的具体实现来使用, 也可以将其作为一个实现了 TraitA 的类型来使用。




