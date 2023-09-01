fn main() {
    let number = plus_one(five() - 3);
    condition(true);
    condition(false);
    if number > 0 {
        println!("Is below 5? {}", control(number));
        another_fn(number, h())
    } else {
        another_fn(0, 'Z')
    }
}

fn control(number: i32) -> &'static str {
    if number < 5 {
       "yes"
    } else {
        "no"
    }
}

fn condition(input: bool) {
    let number = if input { 5 } else { 6 };
    println!("number: {number}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_fn(x: i32, label: char) {
    println!("The value of x is: {x}{label}");
}

fn five() -> i32 {
    5
}

fn h() -> char {
    'h'
}
