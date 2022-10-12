use std::io::stdin;

fn main(){
    
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Input error!");
    
    let p: u8 = input.as_bytes()[0];
    let p1: char = p as char;

    let r: u8 = input.as_bytes()[2];
    let r1: char = r as char;
    
    if p1=='0' {
        println!("C");
    } else {

        if r1=='1'{
            println!("A");
            
        } else {

            println!("B");
        }
    }

    
}
