use std::io;

pub fn main() {
    let mut cont=0;

    loop {
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Input error!");

        let steps: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        if steps == -1 {
            break;
        }

        cont += 1;

        let cycle: i32 = steps/2;
        
        println!("Experiment {}: {} full cycle(s)", cont, cycle);


    }
}