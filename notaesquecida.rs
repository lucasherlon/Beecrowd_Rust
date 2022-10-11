use std::io::stdin;

fn main(){

    let mut nota1 = String::new();
    let mut media = String::new();

    stdin().read_line(&mut nota1).expect("Erro de input");
    stdin().read_line(&mut media).expect("Erro de input");

    let nota1: i32 = match nota1.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let media: i32 = match media.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let nota2 = media*2-nota1;

    println!("{}", nota2);

}