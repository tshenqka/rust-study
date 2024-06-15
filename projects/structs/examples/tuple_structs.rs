struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let black = Color(0, 0, 0); // they are inherently different types
    let origin = Point(0, 0, 0); // but like tuples you can destructure them and use . to access index

    let subject = AlwaysEqual;
}