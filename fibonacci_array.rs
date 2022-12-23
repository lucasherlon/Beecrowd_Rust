use std::io;

fn main() {
    let mut array: [u64; 61] = [1;61];
    array[0] = 0;
    array[1] = 1;

    for index in 2..61 {
        array[index] = array[index -1] + array[index - 2];
    }

    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();
    let t: u8 = match t.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };

    let mut n: u8 = 0;

    while n < t {
        let mut indice = String::new();
        io::stdin().read_line(&mut indice).unwrap();
        let indice: usize = match indice.trim().parse() {
            Ok(num) => num,
            Err(_) => 0
        };

        println!("Fib({}) = {}", indice, array[indice]);
        n += 1;
    }

}