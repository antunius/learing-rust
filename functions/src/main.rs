fn main() {
    println!("Hello, world!");
    another_function();
}

fn another_function() {
    println!("Other Function");
    println!("return {}", parameter_function_and_return(5));
}

fn parameter_function_and_return(x: i32) -> i32 {
    let y = {
        x + 1
    };
    println!("x  = {} and y = {}", x, y);
    return y;
}
