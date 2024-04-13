pub fn print_characters() {
    for char1 in 'a'..='z' {
        println!("{}", char1);
        for char2 in 'A'..='z' {
            println!("{}", char2);
        }
    }
}

fn main() {
    print_characters();
}