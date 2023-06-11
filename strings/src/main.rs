fn main() {
    let s = "String"; // is a &str
    let mut string = String::from(s);

    string.push_str(" mutable!");

    let new_string = string.clone();
    println!("{} {}", string, new_string)
}
