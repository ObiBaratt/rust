use std::io;

fn main() {
    loop {
        println!("Input a temp in F to convert to C.");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("~{n} bytes read~");
            },
            Err(error) => println!("Error: {error}"),
        }

        match input.trim().parse::<i32>() {
            Ok(temp) => {
                let celsuis = (temp - 32) * 5/9;
                println!("{}F is {}C", temp, celsuis);
                break
            }
            Err(_) => continue,
        };
    }
}
