use std::io::{Write, stdin, stdout};

fn main() {
    // loop {
    //     println!("Hello...");
    // }

    // let mut counter: u8 = 0;

    // loop {
    //     counter += 1;
    //     println!("{counter}");
    //     if counter == 100 {
    //         break;
    //     }
    // }

    print!("Please enter a number: ");
    stdout().flush().unwrap();
    let mut input = String::new();
    // let number: u8 = std::io::stdin().read_line(&mut input).expect("ERROR");
    stdin().read_line(&mut input).expect("ERROR");
    let number: u64 = input.trim().parse().expect("ERROR");
    let mut counter = number;
    loop {
        counter -= 1;
        print!("{counter}, ");
        if counter == 0 {
            break;
        }
    }
}
