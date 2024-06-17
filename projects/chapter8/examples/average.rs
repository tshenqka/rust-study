// improved version
use std::collections::HashMap;

enum Average {
    Median(f32),
    Mode(i32),
    Mean(f32),
}

fn display_averages(input: &mut [i32]) -> (Average, Average, Average) {
    input.sort_unstable();
    let len = input.len();

    let median = if len % 2 == 0 {
        (input[len / 2 - 1] + input[len / 2]) as f32 / 2.0
    } else {
        input[len / 2] as f32
    };

    let sum: i32 = input.iter().copied().sum();
    let mean = sum as f32 / len as f32;

    let mut counts = HashMap::new();
    for &num in input.iter() {
        *counts.entry(num).or_insert(0) += 1;
    }

    let mode = counts.iter().max_by_key(|&(_, count)| count).map(|(&num, _)| num).unwrap_or(0);

    (
        Average::Median(median),
        Average::Mean(mean),
        Average::Mode(mode),
    )
}

fn main() {
    let mut input1 = vec![1, 2, 3, 4, 5, 1];
    let (median, mean, mode) = display_averages(&mut input1);

    match median {
        Average::Median(val) => println!("Median: {}", val),
        _ => (),
    }
    match mean {
        Average::Mean(val) => println!("Mean: {}", val),
        _ => (),
    }
    match mode {
        Average::Mode(val) => println!("Mode: {}", val),
        _ => (),
    }
}


// // original version

// fn display_averages(input: &mut Vec<i32>) -> (i32, i32) {

//     input.sort();
//     let median = input[input.len() / 2];

//     let mut map = std::collections::HashMap::new();
//     let mut max = 0;
//     for item in input {
//         let count = map.entry(item).or_insert(0);
//         *count += 1;

//         if *count > max {
//             max = *count;
//         }
//     }

//     return (median, max)

// }

// fn main() {
//     let mut input1 = vec![1, 2, 3, 4, 5, 1];
//     let result = display_averages(&mut input1);
//     println!("{:?}", result);
// }