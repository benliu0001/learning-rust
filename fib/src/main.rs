use std::io;

fn main() {
    println!("How many fibonacci numbers do you want to generate?");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    print!("0 1 ");

    let mut x = 0;
    let mut fib = 1;
    for _ in 0..n {
        let prev = fib;
        fib = fib + x;
        x = prev;
        print!("{} ", fib);
    }
}
