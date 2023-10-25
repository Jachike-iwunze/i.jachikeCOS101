fn main(){
    let p:f64 = 210_000.0;
    let n:f64 = 3.0;
    let r:f64 = 5.0;

    // now to use the formula for cl for depreciation
    let o = 1.0 - (r / 100.0);
    let g = f64:: powf(o,n);
    let d = p * g;
    println!("The value of the TV after 3 years is {}", d);








}