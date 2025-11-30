use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() 
{
    loop 
    {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("{secret_number}");


    println!("Enter a number between 1-100");

    
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess:u32 = match guess.trim().parse() {
        Ok(guess) => guess,
        Err(_) => continue,
    };
    println!("Hmm, you guessed {guess}");


    match guess.cmp(&secret_number)
    {
        Ordering::Less => println!("Lol you're way too low"),
        Ordering::Greater => println!("Lol now you're way too high"),
        Ordering::Equal => {
            println!("The probablility that you'd win is 1 in a 100, yet here you are\nYou win!");
            break;
        }
    }

}
}
