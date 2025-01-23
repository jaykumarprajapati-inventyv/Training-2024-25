#[derive(Debug)]
enum IpAddrKind {
    v4(String),
    v6(String),
}

fn main() {
    let home = IpAddrKind::v4(String::from("123.12.11.5"));
    route(home);
}

fn route(ip: IpAddrKind) {
    println!("Ip address is:{ip:?}");
}
