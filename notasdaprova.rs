use std::io::stdin;

fn main(){

    let mut nota = String::new();

    stdin().read_line(&mut nota).expect("Erro de input");

    let nota: i32 = match nota.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    if nota==0{
        println!("E");
    } else if nota>=1 && nota<=35{
        println!("D");
    } else if nota>=36 && nota<=60{
        println!("C");
    } else if nota>=61 && nota <=85{
        println!("B");
    } else{
        println!("A");
    }
    
}