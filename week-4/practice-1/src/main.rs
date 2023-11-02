fn main() {
    let name = "Aisha Lawal";
    let uni:&str = "Pan-Atlantic University";
    let addr:&str = "km 52 Lekki-Epe Expressway, Igbeju-Lekki, Lagos";
    println!("Name: {}", name );
    print!("University: {}, \n Address: {}",uni,addr);




    let department:&'static str = "Computer Science";
    let school:&'static str = "School of Science and Technology";

    println!("Department: {}, \n school:{}",department, school );
}
