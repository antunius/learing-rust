fn main() {
    let number = 8;
    if number % 4 == 0 {
        println!("Divisible by 4")
    } else if number % 3 == 0 {
        println!("Divisible by 3")
    } else if number % 2 == 0 {
        println!("Divisible by 2")
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let x = if number % 2 == 0 {
        7
    } else { 9 };

    println!("x = {}", x);

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("result {result}");

    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // TODO ^ Try adding 13 to the list of prime values
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
        // TODO ^ Try commenting out this catch-all arm
    }
}
