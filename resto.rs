use std::io::stdin;

fn main(){
    
    let mut num1 = String::new();
    let mut num2 = String::new();

    stdin().read_line(&mut num1).expect("Erro de input");
    stdin().read_line(&mut num2).expect("Erro de input");

    let num1: i32 = match num1.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };
    
    let num2: i32 = match num2.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };

    let res = num1%num2;

    
    println!("{}", res);

}
