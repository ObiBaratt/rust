fn main() {
    let n = 150;
    println!("{n} position of the Fibonacci sequence: {}", fibonacci(n));
}

fn fibonacci(n: u128) -> u128 {
    // edge case for 1 and 2
    if n == 0 {return 0;}
    if n == 1 {return 1;}
    // the sum of the prev 2 numbers
    let mut cur = 1;
    let mut prev_prev = 0;
    let mut prev = 1;

    loop {
        if cur == n {
            return prev;
        }
        let temp = prev;
        prev = prev + prev_prev;
        prev_prev = temp;
        cur += 1;
    }

}
