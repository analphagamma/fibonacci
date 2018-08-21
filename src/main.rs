use std::io;

fn main() {
    println!("Which element do you want?");
    let mut elem = String::new();
    io::stdin().read_line(&mut elem)
        .expect("Could not read");

    let elem:u64 = elem
        .trim()
        .parse()
        .expect("Not a number!");

    println!("Fibonacci value: {}", fibo(elem));
    
}

fn fibo(n: u64) -> u64 {

    match n {
        0 => 0,
        1 => 1,
        _ =>  fibo(n - 1) + fibo(n - 2),
    }
}
