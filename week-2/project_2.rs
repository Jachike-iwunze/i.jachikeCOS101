fn main(){
    let t:f64 = 450_000.00;
    let m:f64 = 1_500_000.00;
    let h:f64 = 750_000.00;
    let d:f64 = 2_850_000.00;
    let a:f64 = 250_000.00;

    // put the quantity

    let z:f64 = 2.0;
    let y:f64 = 1.0;
    let x:f64 = 3.0;
    let w:f64 = 3.0;
    let v:f64 = 1.0;

    // to find total  sales

    let ts = (t * z) + (m * y) + (h * x) + (d * w) + (a * v);
        println!("Total sum of sales {}", ts);

        // the total quantity
        let tq = z + y + x + w + v;
            println!("Total sum of the quantity is {}", tq);

    //now we find the average

    let a = ts / tq;
    println!("the avearage of the sales is {}", a);


}