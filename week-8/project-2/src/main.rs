use std::io;

fn num_of_siblings() -> i32 {
    let mut input = String::new();
    println!("How many siblings do you have");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: i32 = input.trim().parse().expect("Failed to read input");
    n
}

fn main() {
    let mut myarr:Vec<String> = Vec::new();
    let n = num_of_siblings();

    for i in 0..n {
        let mut name = String::new();
        println!("Name of sibling ({}):", i + 1);  // Fixed the indexing issue and added a missing semicolon
        io::stdin().read_line(&mut name).expect("Failed to read input");
        let name = name.trim();
        myarr.push(name.to_string());

        let mut age = String::new();
        println!("Age of sibling ({}):", i + 1);
        io::stdin().read_line(&mut age).expect("Failed to read input");
        let age:i32 = age.trim().parse().expect("Failed to read input");
        myarr.push(age.to_string());

        if age > 18 {
            let mut status = String::new();
            println!("married or single: ");
            io::stdin().read_line(&mut status).expect("Failed to read input");
            let status = status.trim().to_lowercase();
            myarr.push(status.to_string());

            if status == "single"{
                let mut occupation = String::new();
                println!("student or worker: ");
                io::stdin().read_line(&mut occupation).expect("Failed to read input");
                let occupation = occupation.trim().to_lowercase();
                myarr.push(occupation.to_string());

                if occupation == "student"{
                    let mut university_name = String::new();
                    println!("Name of university: ");
                    io::stdin().read_line(&mut university_name).expect("Failed to read input");
                    let university_name = university_name.trim();
                    myarr.push(university_name.to_string());

                    let mut course_of_study = String::new();
                    println!("Course of study: ");
                    io::stdin().read_line(&mut course_of_study).expect("Failed to read input");
                    let course_of_study = course_of_study.trim();
                    myarr.push(course_of_study.to_string());
                }

            }
            else if status == "married"{

                let mut offspring_status = String::new();
                println!("Any offring? (yes/no)");
                io::stdin().read_line(&mut offspring_status).expect("Failed to read input");
                let offspring_status = offspring_status.trim();
                myarr.push(offspring_status.to_string());

                let mut family_city = String::new();
                println!("Family city of residence: ");
                io::stdin().read_line(&mut family_city).expect("Failed to read input");
                let family_city = family_city.trim();
                myarr.push(family_city.to_string());
            }
        }
        else if age <= 18{
            let mut waec_status = String::new();
            println!("Written WAEC? (yes/no)");
            io::stdin().read_line(&mut waec_status).expect("Failed to read input");
            let waec_status = waec_status.trim().to_lowercase();
            myarr.push(waec_status.to_string());

            if waec_status == "yes"{
            let mut schl_name = String::new();
            println!("Name of Secondary School: ");
            io::stdin().read_line(&mut schl_name).expect("Failed to read input");
            let schl_name = schl_name.trim();
            myarr.push(schl_name.to_string());
        }
            else if waec_status == "no"{
                let mut class_level = String::new();
                
                println!("What is your current class level");
                io::stdin().read_line(&mut class_level).expect("Failed to read input");
                let class_level = class_level.trim();
                myarr.push(class_level.to_string());
            }


        }



        for val in myarr.iter() {
         println!("{:?}",val );

        }



        
    }
}
