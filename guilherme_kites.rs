use std::io;

pub fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Input error!");

    let lados: u64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let diagonais: u64 = (lados*(lados-3))/2;

    println!("{}", diagonais);
}