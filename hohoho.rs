use std::io::stdin;

pub fn main() {
    
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Input error!");

    let num: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    
    for i in 0..num {
        if i != num-1 {
            print!("Ho ");
        } else {
            println!("Ho!");
        }
    }
    
}