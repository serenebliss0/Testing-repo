use std::io;
use rand::Rng;

fn main()
{
    println!("Guess the number!");

    println!("Please input your guess.");
    
    let mut input_1 = String::new();
    io::stdin().read_line(&mut input_1).expect("Enter a valid number");
    let mut guess:u64 = input_1.trim().parse().expect("Enter a valid number bro!");

    println!("You guessed {}", guess);
    println!("Lets see if you're right");

    let secret_number:u64 = rand::random::<u64>();


    
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)g
        .expect("Failed to read line");
    let mut value:String = guess.trim().parse().to_lowercase().expect("Error");
    
    
    if value == value
    {
        println!("You guessed {}", guess);
    }
    else
    {
        println!("Try again");
    }

    
}