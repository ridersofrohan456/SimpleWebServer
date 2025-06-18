use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("guess number");

    println!("input:");

    let secret_num = rand::rng().random_range(1..=100);
    
    loop {

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("guessed: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("nice");
                break;
            }
        }
    }
}
