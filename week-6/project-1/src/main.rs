use std::io;

fn main() {
   let number_of_voters = 150;
   let mut voters_counter = 0;

    while voters_counter < number_of_voters {
        let mut input_1 = String::new();
        let mut input_2 = String::new();
        let mut input_3 = String::new();
        let mut input_4 = String::new();
        let mut input_5 = String::new();
        let mut input_6 = String::new();
        let mut input_7 = String::new();


        println!("Enter your name ");
        io::stdin().read_line(&mut input_1).expect("Failed to read input");

        println!("Enter your email");
        io::stdin().read_line(&mut input_2).expect("Failed to read input");
        
        println!("Enter your department");
        io::stdin().read_line(&mut input_3).expect("Failed to read input");

        println!("Enter state of origin");
        io::stdin().read_line(&mut input_4).expect("Failed to read input");

        println!("Enter you level");
        io::stdin().read_line(&mut input_5).expect("Failed to read input");
        let level:i32 = input_5.trim().parse().expect("Input is not a number");

        println!("Enter you CGPA");
        io::stdin().read_line(&mut input_6).expect("Failed to read input");
        let CGPA :f32 = input_6.trim().parse().expect("Input is not a number");

        println!("Enter status  ' class rep ' or ' not a class rep: ' ");
        io::stdin().read_line(&mut input_7).expect("Failed to read input");
        let candidate_status:String = input_7.trim().parse().expect("Invalid status");


        if CGPA >= 4.0 && level > 100 && candidate_status == "class rep"{
            println!("You are eligible to vote");

        } else {
            println!("You are no eligible to vote");
        }

        println!("Candidate's Information");
        println!("Name: {}",input_1.trim());
        println!("Email: {}", input_2.trim());
        println!("Department: {}",input_3.trim());
        println!("State of origin: {}",input_4.trim());
        println!("Level: {}", input_5.trim());
        println!("CGPA: {}", input_6.trim());







    }


















}
