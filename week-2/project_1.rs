fn main(){
    let p:f64 = 520_000_000.0;
    let n:f64 = 5.0;
    let r:f64 = 10.0;

        // we want to find compound interest so we calculate the value of the amount

        let z = 1.0 + (r / 100.0);
        let y = f64:: powf(z,n);
        let a = p * y;
        println!("Amount is {}", a);

        //now to calculate the compound interest 

        let ci = a - p;
        println!("Compound interest is {}", ci);




}