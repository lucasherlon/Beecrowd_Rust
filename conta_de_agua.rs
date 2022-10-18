use std::io;

pub fn main() {
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Input error!");

    let consumo: u16 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let conta: u16;

    if consumo <= 10 {
        conta = 7;
    } else if consumo <=30 {
        conta = 7 + (consumo-10);
    } else if consumo <= 100 {
        conta = 27 + (consumo-30)*2;
    } else {
        conta = 167 + (consumo-100)*5;
    }

    println!("{}", conta);

}