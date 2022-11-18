use std::io;

fn main(){
	const PI: f64 = 3.14159;
	let area: f64;
	let r: f64;

	let mut raio = String::new();

	io::stdin().read_line(&mut raio).expect("input error!");
	r = raio.trim().parse::<f64>().expect("typer conversion error!");

	area = PI * r.powf(2.);

	println!("A={:.4}", area);
}