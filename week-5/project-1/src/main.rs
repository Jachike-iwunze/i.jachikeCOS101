use std::io;

fn main() {
    // enter the values of a, b, and c
    println!("Enter the value of a: ");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Failed to read line");
    let a: f64 = a.trim().parse().expect("Invalid input. Please enter a number.");

    println!("Enter the value of b: ");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Failed to read line");
    let b: f64 = b.trim().parse().expect("Invalid input. Please enter a number.");

    println!("Enter the value of c: ");
    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("Failed to read line");
    let c: f64 = c.trim().parse().expect("Invalid input. Please enter a number.");

    //the discriminant 
    let discriminant = b * b - 4.0 * a * c;

    // Check the discriminant to determine the nature of the roots
    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("The roots are real and distinct: Root1 = {:.2}, Root2 = {:.2}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("The root is real and equal: Root = {:.2}", root);
    } else {
        let real_part = -b / (2.0 * a);
        let imaginary_part = (-discriminant).sqrt() / (2.0 * a);
        println!("The roots are complex: Root1 = {:.2} + {:.2}i, Root2 = {:.2} - {:.2}i", real_part, imaginary_part, real_part, imaginary_part);
    }
}