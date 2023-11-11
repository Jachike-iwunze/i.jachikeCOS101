
    use std::io;

fn main() {
    // Prompt the user for input
    println!("Enter employee's experience (experienced or inexperienced):");
    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("Failed to read line");
    let experience = experience.trim();

    println!("Enter employee's age:");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read line");
    let age: i32 = age.trim().parse().expect("Invalid age input");

    // Calculate the annual incentive
    let annual_incentive = match experience {
        "experienced" => {
            if age >= 40 {
                1560000
            } else if age >= 30 && age < 40 {
                1480000
            } else if age < 28 {
                1300000
            } else {
                0
            }
        }
        "inexperienced" => 100000,
        _ => {
            println!("Invalid experience input");
            return;
        }
    };

    // Display the annual incentive
    println!("Annual Incentive: N{}", annual_incentive);
}

