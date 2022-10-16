use std::io;

pub fn prime(n: i32) -> bool {
    let tam = (n+1)/2;

    for i in 2..tam+1 {
        if n%i == 0 {
            return false;
        }
    }

    return true;
}

pub fn tenprimes(n: i32) -> i32 {
    
    let mut cont = 0;
    let mut sum = 0;
    let mut num = n;

    while cont<10 {
        if prime(num) {
            sum += num;
            cont+= 1;
        }

        num += 1;
    }

    sum

}

pub fn hours(n: i32) -> i32 {
    60_000_000/n
}

pub fn days(n: i32) -> i32 {
    hours(n)/24
}



pub fn main() {
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Input error!");

    let number: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let total: i32 = tenprimes(number);

    println!("{} km/h", total);
    println!("{} h / {} d", hours(total), days(total));

}