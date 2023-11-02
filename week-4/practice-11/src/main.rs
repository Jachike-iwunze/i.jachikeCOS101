fn main() {
    let a:i32 = 2; //bit presentation 10
    let b:i32 = 3; // bit presenttion 11

    let mut result:i32;

    result = a & b;
        println!("(a & b) => {}", result);

    result = a | b;
        println!("(a | b) => {}", result);

    result = a ^ b;
        println!("(a ^ b) => {}", result);

    result = !b;
        println!("( !b ) => {}", result);

    result = a << b;
        println!("(a << b) => {}", result);

    result = a >> b;
        println!("(a >> b) => {}", result);


    
}
