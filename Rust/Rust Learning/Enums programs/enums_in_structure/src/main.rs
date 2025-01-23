#[derive(Debug)]

enum IpAddKind {
    v4,
    v6,
}

struct IpAdd {
    address: String,
    kind: IpAddKind,
}
impl IpAdd {
    fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
            kind: IpAddKind::v4,
        }
    }
}

fn main() {
    println!("Hello, world!");

    let google_address = IpAdd::new("1.2.3.4");
    let loopback = IpAdd::new("::1");
    println!("Address using loopback:-{:?}", loopback.address);

    demo(google_address);
}

fn demo(google_address: IpAdd) {
    println!("My Ip address is:{}", google_address.address);
    println!("My Ip address version is:{:?}", google_address.kind);
}
