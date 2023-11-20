use std::io;


fn main() {
    let mut researcher_number = String::new();
    let mut researcher_name = String::new();
    let mut number_of_papers = String::new();


    println!("Hello candidate, what's your number? ");
    io::stdin().read_line(&mut researcher_number).expect("Failed to read input");
    let researcher_number:i32 = researcher_number.trim().parse().expect("Failed to read input");

    if researcher_number > 500{
        println!("Only the first 500 can participate");
        return;
    } 

    else{
        println!("What is your name? ");
        io::stdin().read_line(&mut researcher_name).expect("Failed to read input");
        let researcher_name = researcher_name.trim();
    } 
    println!();

    println!("How many papers have you published? ", );
    io::stdin().read_line(&mut number_of_papers).expect("Failed to read input");
    let number_of_papers:i32 = number_of_papers.trim().parse().expect("Failed to read input");

    if number_of_papers >=3 && number_of_papers <5{
        println!("Your incentive is N500,000");
    }

    else if number_of_papers >5 && number_of_papers <10{
        println!("Your incentive is N800,000");
    }

    else if number_of_papers >10{
        println!("Your incentive is N1,000,000");
    }

    else if number_of_papers <3{
        println!("Your incentive is N100,000");
    }


    


}