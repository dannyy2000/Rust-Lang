use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the word!");

    // let secret_number = rand::thread_rng().gen_range(1..=100);
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
        println!("input the number ");

        let mut guess = String::new();


        io::stdin()
            .read_line(&mut guess)
            .expect("Filed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };

        println!("This is your guess {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("Equal");
                break;
            }
        }
    }
}
