fn main() {
    countdown();
    i_stop();
    for_loop();

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = to_ten();

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
 println!("End count = {count}");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn to_ten() -> i32 {
    let mut counter = 0;
    loop {
        counter += 1;

        if counter == 10 {
            break counter ;
        }
    }
}

fn countdown() {
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}


fn i_stop() {
    let mut x = 0;
    'a: loop {
        x += 1;
        'b: loop {
            if x > 10 {
                continue 'a;
            } else {
                break 'b;
            }
        }
        break;
    }
    println!("end x: {x}");
}
