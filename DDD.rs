use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Input error");
    let ddd: u8 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };

    match ddd {
        61 => println!("Brasilia"),
        71 => println!("Salvador"),
        11 => println!("Sao Paulo"),
        21 => println!("Rio de Janeiro"),
        32 => println!("Juiz de Fora"),
        19 => println!("Campinas"),
        27 => println!("Vitoria"),
        31 => println!("Belo Horizonte"),
         _ => println!("DDD nao cadastrado")
    };
}
