use rand::Rng;
use std::cmp;
use std::io;

fn main() {
    let random = rand::thread_rng().gen_range(0..50);
    println!("Guess the number!");

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&random) {
            cmp::Ordering::Less => println!("Too small..."),
            cmp::Ordering::Equal => break,
            cmp::Ordering::Greater => println!("Too big..."),
        }
    }

    println!("You guessed correctly!")

    // Convert the string to a number
    // let guess2: i32 = guess.trim().parse().expect("Is not a number!");

    // if guess2.eq(&random) {
    //     println!("You guessed correctly!");
    // } else {
    //     println!("NO the correct guess")
    // }
}
