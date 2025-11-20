use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Hello, world!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    println!("The secret number is generated. It's between 1 to 10");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //e Parsing am integer out of a string..
        let guess: u32 = match guess.trim().parse() {
            //e Removes whitespace from the beggining and end of a string..
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            //e Cmp method compares two values and can be called on anything that can be compared..It takes a reference to whatever you want to compare with: Here it's comparing guess to secret number
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
