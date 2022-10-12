use std::io::stdin;

fn main(){

    let mut seq = String::new();
    let mut seq2 = String::new();

    stdin().read_line(&mut seq).expect("Input error");
    stdin().read_line(&mut seq2).expect("Input error");

    if seq<=seq2 {
        print!("{}{}", seq, seq2);
    } else {
        print!("{}{}", seq2, seq);
    }

}