use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    
    let l: i32;
    let d: i32;
    let k: i32;
    let p: i32;

    io::stdin().read_line(&mut input1).expect("inout error!");
    let vet1: Vec<i32> = input1.split(" ")
    .map(|x| x.trim().parse::<i32>().expect("parsing error!"))
    .collect();

    io::stdin().read_line(&mut input2).expect("inout error!");
    let vet2: Vec<i32> = input2.split(" ")
    .map(|x| x.trim().parse::<i32>().expect("parsing error!"))
    .collect();

    l = vet1[0];
    d = vet1[1];
    k = vet2[0];
    p = vet2[1];

    let custo_percurso: i32 = l *k;
    let custo_pedagio: i32 = (l / d) * p;
    let custo_total: i32 = custo_pedagio + custo_percurso;

    println!("{}", custo_total);
}