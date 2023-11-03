use std::io;

fn main() {
    // Prompt the user to enter the distance in miles and time in hours
    println!("Enter the distance in miles: ");
    let mut distance = String::new();
    io::stdin().read_line(&mut distance).expect("Failed to read line");
    let distance: f64 = distance.trim().parse().expect("Invalid input. Please enter a number.");

    println!("Enter the time in hours: ");
    let mut time = String::new();
    io::stdin().read_line(&mut time).expect("Failed to read line");
    let time: f64 = time.trim().parse().expect("Invalid input. Please enter a number.");

    // Calculate the speed in kilometers per hour
    let speed_kph = (distance * 1.60934) / time;

    // Print the result
    println!("The car is traveling at {:.2} kilometers per hour.", speed_kph);
}