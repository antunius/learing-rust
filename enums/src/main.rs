
#[derive(Debug)]
enum IpAddrKing {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKing,
    address: String,
}

fn main() {
    let v4 = IpAddrKing::V4;
    let home = IpAddr {
        kind: IpAddrKing::V4,
        address: String::from("127.0.0.1"),
    };
    println!("{:?}", home);

    match v4 {
        IpAddrKing::V4 => println!("IpV4"),
        IpAddrKing::V6 => println!("IpV6"),

    }
}
