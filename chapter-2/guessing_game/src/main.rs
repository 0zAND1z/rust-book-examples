extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("***GUESS THE NUMBER ***");

    let secret_num = rand::thread_rng().gen_range(1, 4);
    // println!("Secret number generated: {}", secret_num);

    loop {
        println!("Enter a number: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too greater!"),
            Ordering::Equal => {
                println!("~~~Congrats! Same Number.~~~");
                break;
            }
        }
    }
}
