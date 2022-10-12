use std::io::stdin;

fn main(){
    
    let mut lados = String::new();

    stdin().read_line(&mut lados).expect("Input error!");

    let lados: i32 = match lados.trim().parse() {

        Ok(num) => num,
        Err(_) => 0,
    };

    let triangulos = lados - 2;

    println!("{}", triangulos);

}
