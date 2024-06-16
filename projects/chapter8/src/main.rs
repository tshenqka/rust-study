fn main() {

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let mut v1 = vec![100, 25, 57];
    for n_ref in &v1 {
        let n_plus_one = *n_ref + 1;
        println!("{n_plus_one}");
    }

    let mut s = v[0]; // this actually wouldnt compile if v stored strings. Cant leave vector in inconsistent state

    for n_ref in &mut v1 {
        *n_ref += 50;
    }



    let mut v2 = vec![1, 2];
    let mut iter = v.iter();
    let n1: &i32 = iter.next().unwrap();
    let n2: &i32 = iter.next().unwrap();
    let end: Option<&i32> = iter.next();

    let mut v3 = vec![1, 2];
    let mut iter = 0 .. v3.len();
    let i1 = iter.next().unwrap();
    let n1 = &v[i1];

    enum SpreadsheetCell { // this is an enum
        Int(i32), // this is a variant
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ]


}
