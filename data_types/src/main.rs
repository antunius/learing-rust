fn main() {
    //integer
    let x: u32 = "42".parse().expect("Expected a u32 number");
    println!("x = {}", x);

    //floating point

    let mut y = 5.6;
    y = "67.4".parse().expect("Expected f64 floating point");
    println!("{}", y);

    // bool
    let t = true;

    //char

    let c = 'v';

    //tuple

    let tup: (i32, f64, bool) = (3, 5.6, true);
    println!("tuple = {:?}", tup);

    //array

    let a = [1, 2, 3, 4];

    println!("array = {:?}", a);
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    println!("months array = {:?}", months);

    println!("First month = {}", months[0])
}
