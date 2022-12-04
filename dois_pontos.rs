use std::io;

fn distancia(p1x: f64, p1y: f64, p2x: f64, p2y: f64) -> f64 {
    ((p2y - p1y).powf(2.) + (p2x - p1x).powf(2.)).sqrt()
}

fn main() {
    let mut p1 = String::new();
    let mut p2 = String::new();

    io::stdin().read_line(&mut p1).expect("input error!");
    let pt1: Vec<f64> = p1.split(" ")
    .map(|x| x.trim().parse::<f64>().unwrap())
    .collect();

    io::stdin().read_line(&mut p2).expect("input error!");
    let pt2: Vec<f64> = p2.split(" ")
    .map(|x| x.trim().parse::<f64>().unwrap())
    .collect();

    let p1x: f64 = pt1[0];
    let p1y: f64 = pt1[1];

    let p2x: f64 = pt2[0];
    let p2y: f64 = pt2[1];

    let distan = distancia(p1x, p1y, p2x, p2y);
    println!("{:.4}", distan);

}