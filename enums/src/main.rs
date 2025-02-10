
// Version four IP addresses will always have four numeric components that will have values between 0 and 255.
struct Ipv4Addr {
    components: [u8; 4],
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}


fn main() {
    let ip4 = Ipv4Addr { components: [127,0,0,1]};
    let home = IpAddr::V4(ip4);

    let loopback = IpAddr::V6(String::from("::1"));
    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

}
