use std::io::{self, Read};

// Define a struct to represent a user
struct User {
    role: String,
}

// Function to display the database structure based on user role
fn display_database_structure(user: &User) {
    match user.role.as_str() {
        "administrator" => println!("Displaying database structure..."),
        "project manager" => println!("Displaying structure of the project table..."),
        "employee" => println!("Displaying structure of the staff table..."),
        "customer" => println!("Calling the customer table..."),
        "vendor" => println!("Displaying data-plan table..."),
        _ => println!("Invalid role!"),
    }
}

fn main() {
    // Read user role from the console
    println!("Enter your role (administrator/project manager/employee/customer/vendor):");
    let mut role_input = String::new();
    io::stdin().read_line(&mut role_input).expect("Failed to read input");

    // Create a User instance with the entered role
    let user = User {
        role: role_input.trim().to_lowercase(),
    };

    // Display the corresponding database structure based on the user's role
    display_database_structure(&user);

    // Now, let's integrate the code to read a file and display its content
    let mut file = std::fs::File::open("globacom_dbase.sql");
    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            if let Err(err) = file.read_to_string(&mut contents) {
                eprintln!("Error reading file: {}", err);
            } else {
                println!("File contents:\n{}", contents);
            }
        }
        Err(err) => {
            eprintln!("Error opening file: {}", err);
        }
    }
}