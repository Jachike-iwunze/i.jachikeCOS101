fn main() {
    // initialize a multable tuple
    let mut moutain_heights = ("Everest",8848,"Fishtail", 6993);

    println!("original tuple = {:?}", moutain_heights);


    // change 3rd and 4th elements of a lutable tuple
    moutain_heights.2 = "Lhotse";
    moutain_heights.3 = 8516;

    println!("Changed tuple = {:?}", moutain_heights);
}
