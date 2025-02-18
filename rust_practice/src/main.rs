use std::io;
use std::fs;
use std::vec;
use chrono;
use chrono::Local;

// STRUCTS
#[derive(Debug)]
struct User {
    _active: bool,
    _username: String,
    _email: String,
    _sign_in_counts: u64,
}
struct Rect {
    width: u32,
    height: u32,
}
impl Rect {
    fn area(&self) -> u32 {
        return self.height * self.width;
    }
    fn perimeter(&self) -> u32 {
        return 2*self.height*self.width;
    }
}

// ENUMS
#[derive(Debug)]
enum Directions {
    North,
    _South,
    _East,
    _West
}
enum Shape {
    Rectangle(f64,f64),
    Cricle(f64),
}

fn main() {

    // STRINGS
    let str = String::from("123456");
    println!("{}", get_string_len(&str));

    // LOOPS
    let fib_ans = get_fib(5);
    let fib2_ans = fib2(10);
    println!("{}", fib_ans);
    println!("{}", fib2_ans);

    // CONTROL FLOW
    let mut s = String::new();
    println!("please enter a number");
    io::stdin().read_line(&mut s).expect("error reading line!");
    let s = s.trim().parse::<i32>().expect("please enter a number!!");
    println!("{}",is_even(s));

    // STRUCTS
    let user1 = User {
        _active: true,
        _username: String::from("elon musk"),
        _email: String::from("elon@example.com"),
        _sign_in_counts: 2,
    };
    println!("user1 username : {:#?}", user1);
    let rect = Rect {
        height: 44,
        width: 65,
    };
    println!("Area of Rectangle: {}", rect.area());
    println!("Perimeter of Rectangle: {}", rect.perimeter());

    // ENUMS
    let dir = Directions::North;
    println!("{:?}", dir);
    let shape1 = Shape::Cricle(3.5);
    let shape2 = Shape::Rectangle(5.6, 8.8);
    println!("{}", get_enum_area(shape1));
    println!("{}", get_enum_area(shape2));

    // OPTIONS
    let exmp = String::from("john cena");
    match get_first_a(&exmp) {
        Some(val) => println!("first a is at position {}",val),
        None => println!("a not found"),
    };

    // RESULTS
    let text = fs::read_to_string("file.txt");
    match text {
        Ok(val) => println!("contents of the file is :{:?}", val),
        Err(err) => println!("error occured : {:?}", err)
    }

    // CRATES
    let now = chrono::Utc::now();
    println!("current date and time in UTC: {}", now);
    let formatted = now.format("%Y-%m-%d %H:%M:%S");
    println!("current date and time after formatting: {}", formatted);
    let local = Local::now();
    println!("current date and time in local: {}", local);

    // VECTORS
    let mut vec0 = Vec::new();
    vec0.push(1);
    vec0.push(2);
    vec0.push(3);
    vec0.push(4);

    let mut vec = vec![1,2,3,4];
    println!("{:?}", vec);
    // vec = even_filter_vec(vec);
    // let mut vec2 = Vec::new();
    // vec2 = even_filter_vec(&vec);
    let vec2 = even_filter_vec(&vec);
    println!("{:?}", vec2);
    even_values(&mut vec);
    println!("{:?}", vec);

}

// IF-ELSE
fn is_even(num: i32) -> bool {
    if num%2==0 {
        return true;
    } else {
        return false;
    }
}

// IF/WHILE
fn get_fib(num: u32) -> u32{
    let mut a = 0;
    let mut b = 1;
    let mut count = 2;
    if num == 0 {
        return a;
    }
    if num == 1 {
        return b
    }
    // println!("{}",a);
    // println!("{}",b);
    while count<=num {
        let s = a+b;
        // println!("{}",s);
        a = b;
        b = s;
        count += 1;
    }
    return b;
}

// INBUILT-FUNCS
fn get_string_len(s: &str) -> usize{
    return s.chars().count();
}

// IF/FOR
fn fib2(num: u32) -> u32{
    let mut first = 0;
    let mut second = 0;
    if num == 0 {
        return first;
    }
    if num == 1 {
        return second
    }
    for _ in 0..(num-1) {
        let temp = second;
        second = first + second;
        first = temp;
    }
    return second;
}

// PAT-MATCH/ENUMS
fn get_enum_area(shape: Shape) -> f64{
    match shape{
        Shape::Rectangle(a, b) => a*b,
        Shape::Cricle(r) => std::f64::consts::PI*r*r
    }
}

// OPTIONS/ITERATORS
fn get_first_a(s: &str) -> Option<i32> {
    for (index, charecter) in s.chars().enumerate() {
        if charecter=='a' {
            return Some(index as i32);
        }
    }
    return None;
}

// VECTORS
fn even_filter_vec(vec: &Vec<i32>) -> Vec<i32>{
    let mut new_vec = Vec::new();
    for val in vec {
        if val%2==0 {
            new_vec.push(*val);
        }
    }
    return new_vec;
}
fn even_values(vec: &mut Vec<i32>) {
    let mut i = 0;
    while i<vec.len(){
        if vec[i]%2 != 0 {
            vec.remove(i);
        }
        i+=1;
    }
}