use std::io;

fn delta(a: f64, b: f64, c: f64) -> f64 {
    b * b - (4.0 * a * c)
}

fn main () {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Input error!"); 
    let input: Vec<f64> = input.split(" ")
    .map(|x| x.trim().parse::<f64>().expect("Parsing error!"))
    .collect();

    let a = input[0];
    let b = input[1];
    let c = input[2];
    let delta: f64 = delta(a,b,c);

    if delta < 0.0 || a == 0.0 {
        println!("Impossivel calcular");
    } else if delta == 0.0 {
        let raiz = (-b + delta.sqrt()) / (2.0 * a);
        println!("R1 = {:.5}", raiz );
    } else {
        let raiz1 = (-b + delta.sqrt()) / (2.0 * a);
        let raiz2 = (-b - delta.sqrt()) / (2.0 * a);
        println!("R1 = {:.5}", raiz1 );
        println!("R2 = {:.5}", raiz2 );
    }
}