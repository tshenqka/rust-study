enum IpAddr {
    V4(String),
    V6(String), // associated string values
}

fn main() {

    let home = IpAddr::V4(String::from("127.0.0.1")); // automatically get constructor defined as a result of using enum

    return;
}