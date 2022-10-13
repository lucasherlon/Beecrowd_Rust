use std::io::stdin;

pub fn main() {
    
    let mut pos = String::new();

    stdin().read_line(&mut pos).expect("input error");

    let pos: u32 = match pos.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let real_pos: u32 = match pos {
        1 => 1,
        2..= 3 => 3,
        4..= 5 => 5,
        6..= 10 => 10,
        11..= 25 => 25,
        26..= 50 => 50,
        51..= 100 => 100,
        _ => panic!("unexpected position"),
    };

    println!("Top {}", real_pos);

}