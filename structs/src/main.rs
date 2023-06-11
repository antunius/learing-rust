#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn main() {
    let ret = Rectangle {
        width: 40.0,
        height: 60.3,
    };

    println!("rectangle = {:?} area = {}", ret, ret.area())
}
