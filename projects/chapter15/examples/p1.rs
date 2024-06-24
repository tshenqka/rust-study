// enum List {
//     Cons(i32, List),
//     Nil,
// } // in this example we get an error suggesting indirection: store a pointer to the value instead


enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    return;
}