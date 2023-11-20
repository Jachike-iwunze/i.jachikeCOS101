
use std::io;

fn main() {
    println!("Input the value of n");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let input1:u32 = input1.trim().parse().expect("Failed to read input");

    for x in 1..=input1 {
        for y in 1..=12{
            let result = x * y;
            println!("{} x {} = {}",x, y, result);
        }
        println!();
    }
}
