enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String), // associated string values
}

fn main() {

    let home = IpAddr::V4(127, 0, 0, 1); // automatically get constructor defined as a result of using enum
    let loopback = IpAddr::V6(String::from("::1"));

    return;
}