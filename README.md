## Rust笔记文档


参考资料：

Rust中文教程

霍丙乾 bennyhuo 


## 案例一 改进字符计数器

```rust
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;


#[derive(Debug)]
struct WordCounter(HashMap<String, u64>);

impl WordCounter {
    fn new() -> WordCounter {
        WordCounter(HashMap::new())
    }
  
  
    
    fn increment(&mut self,word: &str) {  
        let key = word.to_string();
        let count = self.0.entry(key).or_insert(0);
        *count += 1;
    }
    
     
    fn display(&self) {
        for (key, value) in self.0.iter() {
            println!("{}: {}", key, value);
        }
    }
    
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let filename = &arguments[1];
    
    println!("Processing file: {}", filename);
    
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut word_counter = WordCounter::new();
    
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let words = line.split_whitespace(); 
        for word in words {
            if word.is_empty() {
                continue
            } else {
                word_counter.increment(word);
            }
        }
    }
    
    word_counter.display();
}


```
