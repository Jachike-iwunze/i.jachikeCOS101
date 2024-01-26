use std::fs::File;
use std::io::{self, Write};

struct Company {
    name: String,
    shares: u32,
    liabilities: u32,
    year: u32,
}

impl Company {
    fn calculate_leverage(&self) -> f32 {
        ((self.shares as f32 - self.liabilities as f32) / self.shares as f32) * 100.0
    }
}

fn valid_password(password: &str) -> bool {
    password.len() >= 3
        && password.len() <= 8
        && password.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        && !password.chars().any(|c| "@$#".contains(c))
}

// Save the leverages into a file
fn put_leverage_into_file(companies: &[Company]) {
    let mut file = File::create("company-Leverages.txt").expect("Failed to create");

    writeln!(file, "\t\tCOMPANY LEVERAGES").expect("Failed to write");
    writeln!(file, "\t\t--------------\n").expect("Failed to write");

    for company in companies {
        if company.shares > 20_000_000 {
            let leverage = company.calculate_leverage();
            writeln!(file, "{}: {:.2}%", company.name, leverage).expect("Failed to write");
        }
    }

    println!("Leverages saved to Leverages.txt");
}

fn main() {
    let companies = vec![
        Company {
            name: String::from("Cadbury Nigeria Plc"),shares: 15_000_000,liabilities: 5_500_000,year: 1965,
            
        },
        Company {
            name: String::from("Champion Breweries Plc"),  shares:25_000_000, liabilities:8_000_000,  year:1974,
        },
        Company {
            name: String::from("Dangote Sugar Refinery"),  shares:18_000_000, liabilities:10_000_000, year:1970,
        },
        Company {
            name: String::from("Flour Mills Nigeria Plc"), shares:32_000_000, liabilities:4_000_000,  year:1960,
        },
         Company {
            name: String::from("Honeywell Nigeria Plc"),   shares:34_000_000, liabilities:9_000_000,  year:1906,
        },
       
        Company {
            name: String::from("Nestle Nigeria PLc"),      shares:8_000_000,  liabilities:1_500_000,  year:1961,
        },
         Company {
            name: String::from("Nigerian Breweries Plc"),  shares:30_000_000, liabilities:12_000_000, year:1946,
        }, 
       
        Company {
            name: String::from("Unilever Nigeria Plc"),    shares:37_000_000, liabilities:11_000_000, year:1923,
        },
    ];

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Please Enter your company name");
    io::stdin().read_line(&mut input1).expect("Failed to read");
    let written_company_name = input1.trim();  // Corrected variable name

    println!("Please enter The required username");
    io::stdin().read_line(&mut input2).expect("Failed to read");
    let default_username = &companies[0].name[..4];

    println!("Please type in the password:");
    io::stdin().read_line(&mut input3).expect("Failed to read");
    let password = input3.trim();

    for company in &companies {
        if default_username == &company.name[..4].to_lowercase() {
            // if password is valid
            if !valid_password(password) {
                println!("Invalid password.");
                return;
            }

            println!("Company information saved");
            put_leverage_into_file(&companies);
        }
    }

    println!("Company information saved");

    let leverage = companies[0].calculate_leverage();  // Change to the correct company

    let mut file = std::fs::File::create("company.txt").expect("Failed to create");
    file.write_all("\t\tTHE COMPANY INFORMATION\n".as_bytes()).expect("Failed to write");
    file.write_all("\t\t------------------------\n\n".as_bytes()).expect("Failed to write");

    writeln!(
        file,
        "Company: {}\nFounded: {}\nShares: {}\nLiabilities: {}\nLeverage: {:.2}%",
        companies[0].name,
        companies[0].year,
        companies[0].shares,
        companies[0].liabilities,
        leverage
    )
    .expect("Failed to write");

    if companies[0].shares > 20_000_000 {
        let multiple_of_leverage = leverage * 2.0;
        writeln!(file, "Multiple of percentage Leverages: {:.2}", multiple_of_leverage)
            .expect("Failed to write");
    }

    if companies[0].liabilities < 10_000_000 {
        let five_percent_of_leverage = (5.0 / 100.0) * leverage;
        writeln!(file, "5% of percentage Leverages: {:.2}", five_percent_of_leverage)
            .expect("Failed to write");
    }
}