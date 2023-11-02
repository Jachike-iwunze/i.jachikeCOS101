fn main() {
    
        let  miles = 80.0;
        let  hours = 2.0;
    
        //  1 mile = 1.60934 kilometers
        let kilometers = miles * 1.60934;
    
        // Speed in kilometers per hour = Distance (in kilometers) / Time (in hours)
        let speed_kilos = kilometers / hours;
    
        println!("The car is traveling at {} kilometers per hour.", speed_kilos);


        //if it goes 120 miles in 4 hours 
        let miles = 120.0;
        let hours = 4.0;
    
        //  1 mile = 1.60934 kilometers
        let kilometers = miles * 1.60934;
    
        // Speed in kilometers per hour = Distance (in kilometers) / Time (in hours)
        let speed = kilometers / hours;
    
        println!("The car is traveling at {} kilometers per hour.", speed);
    
    
}
