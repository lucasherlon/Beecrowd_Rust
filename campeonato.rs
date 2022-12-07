use std::io;

fn main() {
	let mut input = String::new();

    io::stdin().read_line(&mut input)
    .expect("input error!");
    
	let vetor: Vec<i32> = input.split(" ")
	.map(|x| x.trim().parse::<i32>().unwrap())
	.collect();

	let cor: i32 = vetor[0]*3 + vetor[1];
	let fla: i32 = vetor[3]*3 + vetor[4];

	if cor > fla {
		println!("C");
	} else if fla > cor {
		println!("F");
	} else {
		if vetor[2] > vetor[5]{
			println!("C");
		} else if vetor[5] > vetor[2] {
			println!("F");
		} else {
			println!("=");
		}
	}
}
