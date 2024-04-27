fn main() {
   let mut v: Vec<i32> = Vec::new();

   let mut v2 = vec![1, 2, 3];

   let v3 = vec!["Hello", "World"];

   v.push(5);

   v.append(&mut v2);

   for e in &mut v{
      *e = *e * 2;
      println!("{e}");
   }

   println!("*******");
   for e in v.iter() {
     println!("{e}");
    }
}