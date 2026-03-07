use std::io;

pub fn is_palindrome(word:String) -> bool
{
    let reversed  = word.chars().rev().collect::<String>();

    if word == reversed {
        true
    } else {
        false
    }
}

fn main()
{
    println!("Welcome! Enter a word and see if its a palindrome or not!");
    let mut input_1 = String::new();
    io::stdin().read_line(&mut input_1).expect("Failed to read line");
    let mut word_to_reverse= input_1.trim().to_lowercase();

    let mut word = word_to_reverse.clone();
    let is_palindrome = is_palindrome(word_to_reverse);

    if is_palindrome == true
    {
        println!("Yes, the word {} is a palindrome!", word);
    }
    else
    {
        println!("Sigh, unfortunately {} is not a palindrome!", word);
    }

}