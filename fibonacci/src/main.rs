use std::io;

// recursive
fn fib(n: u32) -> u32 {
    if n<=1 {
        return n
    }
    fib(n-1) + fib(n-2)
}

fn main() {
    let result;
    let mut input = String::new();
    let n: u32;

    println!("Please input n:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read n");

    n = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {println!("invalid"); 0},
    };

    // TODO: have to n>=0
    result = fib(n-1);
    println!("{ } is nth fibonach number", result);
}
