use std::io;

fn main() {
    // basic();
    // tup(-500);
    array();
}

fn basic() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let y = 5;
    println!("The value of start y is: {y}");

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of inner y is: {y}");
    }
    println!("The value of end y is: {y}");
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    println!("q: {quotient}, t: {truncated}");
}

fn tup(x: i32) {
    let tup: (i32, f32, u8, char, &str) = (x, 6.1, 1, 's', "hello_world");
    let (a, _, c, _, e) = tup;

    let b = tup.1;
    let d = tup.3;

    println!("Tup vals: {a} {b} {c} {d} {e}");
}

fn array() {
    let a = [1, 2, 3, 4, 5];

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index was not a number");

    let element = a[index];

    println!("The value of element at index: {index} is: {element}");
}
