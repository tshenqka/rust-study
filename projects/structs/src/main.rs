

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    print!(
        "The area of the rectanble is {} square pixels",
        rect1.area()
    );

    println!("rect1 is {:?}", rect1);

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale), // return ownership
        height: 50
    };

    dbg!(&rect2);

    println!("rect1 can hold rect2? {}", rect1.can_hold(&rect2));

}

#[derive(Debug)] // this is used to autmatically generate an implementation of Debug trait for the struct
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle { // implementation block, everything here associated with the Rectangle type
    fn area(&self) -> u32 { // we've borrowed self immutable, but we can also take ownership, borrow mutable, just as they can any other param
    // taking ownership of self is rare and used when we transform and dont want user to use original object
        self.height * self.width
    }

    // rust auto dereferencing takes care of method receiver
    // &self is short for self: &Self
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }   

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle { 
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}