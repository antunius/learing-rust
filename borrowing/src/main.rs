fn main() {
    let mut s1 = String::from("aa");
    let len = calculate_length(&s1);
    println!("{}", len);
    change(&mut s1);
    println!("{}", len);
}

fn change(s1: &mut String) {
    s1.push_str("World");
}

fn calculate_length(s1: &String) -> usize {
    s1.len()
}
