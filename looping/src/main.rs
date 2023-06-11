fn main() {
    let array = [1, 2, 3, 4, 5];

    //for
    for i in array {
        println!("{i}")
    }

    for i in (1..4).into_iter() {
        println!("{i}")
    }
}
