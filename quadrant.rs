use std::io;

fn main() {
    'run: loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Input error!");
        let coordinates: Vec<i32> = input.split(" ")
            .map(|x| x.trim().parse::<i32>().expect("Parsing error!"))
            .collect();
        
        let x: i32 = coordinates[0];
        let y: i32 = coordinates[1];

        if x == 0 || y == 0 {
            break 'run;
        }
        if x > 0 && y > 0 {
            println!("primeiro");
        }
        if x < 0 && y > 0 {
            println!("segundo");
        }
        if x < 0 && y < 0 {
            println!("terceiro");
        }
        if x > 0 && y < 0 {
            println!("quarto");
        }
    }
    
}