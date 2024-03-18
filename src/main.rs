use std::cmp::Ordering;
use std::io; // 预导入 predule predule
use rand::Rng; // trait
fn  main() {
    println!("猜数!");
    println!("猜测一个数!");
    println!("猜测两个数");

    let secret_number = rand::thread_rng().gen_range(1, 101); // i32 u32 i64

    loop {
        println!("猜测一个数");

        let mut guess = String::new(); // 获取当前猜测数据
        io::stdin().read_line(&mut guess).expect("无法读取运行");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("您猜测的数是: {}", guess);

        match guess.cmp(&secret_number) {
          Ordering:: Less => println!("Too small"),
            Ordering:: Greater => println!("Too big"),
            Ordering:: Equal => {
                println!("You win");
                break;
            }
        };
    }
}
