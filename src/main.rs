use std::io;

fn main() {
    println!("Generate the nth fibonacci");
    println!("Enter number: ");

    let mut n = String::new();

    io::stdin()
         .read_line(&mut n)
         .expect("Failed to read line");

    let n: u32 = n.trim().parse().expect("Please type a number!"); 

    let nth = gen_fibonacci(n); 

    println!("The {n}th fibonacci number is: {nth}");
}

fn gen_fibonacci(n: u32) -> u32 {
    if n <= 1 { return n; }
    gen_fibonacci(n-1) + gen_fibonacci(n-2)
} 