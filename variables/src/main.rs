fn main() {
    let x: u32 = 5; //immutable
    {
        let x = 6; // shadowing variable
        println!("x  = {}", x)
    }
    let y = 5; // mutable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // const
}
