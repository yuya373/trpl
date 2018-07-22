use std::net::IpAddr;

fn main() {
    // 127.0.0.1 is always valid ip address
    // it is acceptable to use `unwarp`
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    println!("home: {:?}", home);
}
