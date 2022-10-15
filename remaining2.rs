use std::io;

pub fn main() {

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Input error!");

    let number: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    for i in 1..10001 {
        
        if i%number == 2 {
            println!("{}", i);
        }

    }
}