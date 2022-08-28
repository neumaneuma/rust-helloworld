use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        println!("Please input your guess");
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        // let guess: u32 = guess.trim().parse().expect("Please type a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number");
                continue;
            }
        };
        println!("you guessed {guess} ");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        // let condition = true;
        // let number = if condition { 5 } else { 6 };

        // println!("The value of number is: {number}");
        // let mut counter = 0;

        // let result = loop {
        //     counter += 1;

        //     if counter == 10 {
        //         break counter * 2;
        //     }
        // };

        // println!("The result is {result}");
    }
}
