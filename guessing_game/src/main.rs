use std::io;
use rand::Rng;
use std::cmp::Ordering;

// fn main() {
//     let random_number = rand::thread_rng().gen_range(1..=100);
//     loop{
//         let mut s = String::new();
//         println!("please enter a number as your guess!");
//         io::stdin().read_line(&mut s).expect("error reading input");
//         let s = match s.trim().parse::<usize>() {
//             Ok(num) => num,
//             Err(_) => continue
//         };
//         match s.cmp(&random_number) {
//             Ordering::Equal => {
//                 println!("correct!!");
//                 break;
//             },
//             Ordering::Greater => println!("too big!"),
//             Ordering::Less => println!("too less!"),
//         }
//     }
// }

fn main() {
    let random_number = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut input = String::new();
        println!("please enter a number as your guess!");
        io::stdin().read_line(&mut input).expect("error reading input!");
        let input = match input.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue
        };
        match input.cmp(&random_number) {
            Ordering::Equal => {
                println!("Correct!!");
                break;
            },
            Ordering::Greater => println!("Too Big!!"),
            Ordering::Less => println!("Too Less!!")
        }
    }
}