use std::io;

fn main(){
    let mut num_1 = String::new();
    let mut num_2 = String::new();

    let num1: f64;
    let num2: f64;
    let media: f64;

    io::stdin().read_line(&mut num_1).expect("input error!");
    num1 = num_1.trim().parse::<f64>().expect("conversion error!");

    io::stdin().read_line(&mut num_2).expect("input error!");
    num2 = num_2.trim().parse::<f64>().expect("conversion error!");

    media = (num1*3.5 + num2*7.5)/11.;

    println!("MEDIA = {:.5}", media);
}