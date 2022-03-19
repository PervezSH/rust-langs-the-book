#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let ipv4 = IpAddrKind::V4(127, 0, 0, 1);
    let ipv6 = IpAddrKind::V6(String::from("::1"));

    println!("Version 4 Ip address {:?}", ipv4);
    println!("Version 6 Ip address {:?}", ipv6);
}
