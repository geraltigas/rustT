use std::io;

fn main() {
    println!("guess it");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("wrong");
    println!("guess {}", guess);
}
