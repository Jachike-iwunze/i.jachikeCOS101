use std::io;

fn main() {
    // Display the menu
    println!("Menu:");
    println!("P = Poundo Yam/Edinkaiko Soup      - N3,200");
    println!("F = Fried Rice & Chicken           - N3,000");
    println!("A = Amala & Ewedu Soup             - N2,500");
    println!("E = Eba & Egusi Soup               - N2,000");
    println!("W = White Rice & Stew              - N2,500");

    // Input the type of food
    println!("Enter the type of food (P/F/A/E/W):");
    let mut food_type = String::new();
    io::stdin().read_line(&mut food_type).expect("Failed to read line");
    let food_type = food_type.trim();

    // Input the quantity
    println!("Enter the quantity:");
    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).expect("Failed to read line");
    let quantity: i32 = quantity.trim().parse().expect("Invalid quantity input");

    // Calculate total charges based on the selected food type and quantity
    let total_charges = match food_type {
        "P" => 3200 * quantity,
        "F" => 3000 * quantity,
        "A" => 2500 * quantity,
        "E" => 2000 * quantity,
        "W" => 2500 * quantity,
        _ => {
            println!("Invalid food type");
            return;
        }
    };

    // Apply a 5% discount if total charges are greater than N10,000
    let discount = if total_charges > 10000 { 0.05 * total_charges as f64} else { 0.0 };

    // Calculate the final total after applying the discount
    let final_total = (total_charges as f64) - discount;

    // Display the total charges and any applicable discount
    println!("Total Charges: N{}", total_charges);
    if discount > 0.0 {
        println!("5% Discount Applied: -N{}", discount);
    }
    println!("Final Total: N{}", final_total);
}