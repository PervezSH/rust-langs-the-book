#[derive(Debug)] // so we can inspect
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddrKind {
    fn display_ip(&self) {
        println!("Ip address is {:?}", self);
    }
}

fn main() {
    let ipv4 = IpAddrKind::V4(127, 0, 0, 1);
    let ipv6 = IpAddrKind::V6(String::from("::1"));

    ipv4.display_ip();
    ipv6.display_ip();
}
