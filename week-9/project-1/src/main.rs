use std::io::Write;

fn main() {
    let drink1 = "Lager:\n \n33 Export \n Desperados \nGoldberg,\nGulder \nHeineken \n Star\n";
    let drink2 = "\nStout:\n \nLegend \n Turbo king\nWilliams\n";
    let drink3 = "\nNon-Alcoholic:\n\nMaltina \n Amstel Malta \nMalta Gold \nFayrouz\n";


    let mut file = std::fs::File::create("drinks.txt").expect("create failed ");
    file.write_all("Welcome to the bar\n".as_bytes()).expect("write failed ");
    file.write_all(drink1.as_bytes()).expect("write failed");
    file.write_all(drink2.as_bytes()).expect("write failed");
    file.write_all(drink3.as_bytes()).expect("write failed");


    
    println!("\n Data written to file.");



 


}
