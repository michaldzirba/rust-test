use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number: ");
    let mut guess = String::new();

    let secret = rand::thread_rng().gen_range(1, 11);

    loop {
        guess.clear();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line ");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("less"),
            Ordering::Greater => println!("greater"),
            Ordering::Equal => {
                println!("bravo");
                break;
            }
        };
    }

    println!("Hello, world! {}", guess);
}
