use std::io;

pub fn main() {
    let mut n = String::new();
    let mut m = String::new();

    io::stdin().read_line(&mut n).expect("Input error!");
    io::stdin().read_line(&mut m).expect("Input error!");

    let n: i16 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let m: i16 = match m.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let dif: i16 = n-m;

    println!("{}", dif);

}