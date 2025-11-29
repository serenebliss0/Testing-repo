use std::io;

//collect number of siblings

pub fn client_name() -> String
{
    println!("Please enter your name!");
    let mut name_input = String::new();
    io::stdin().read_line(&mut name_input).expect("Failed to read line");
    let name = name_input.trim();
    println!("Your name is {}!", name);

    String::from(name)
}
pub fn sibling_count()
{
    println!("Please enter the number of siblings you have:");
    let mut  number_of_siblings_input = String::new();
    io::stdin().read_line(&mut number_of_siblings_input).expect("Failed to read line");
    let number_of_siblings:u8 = number_of_siblings_input.trim().parse().expect("Enter a valid number");

    if number_of_siblings == 0
    {
        let name = client_name();
        println!("Thank you, {}", name );
        println!("That would be all for today!");
    }
    else if (number_of_siblings > 0)
    {
        loop
        {
            //type in here!
            println!("HI");
            break;
        }
    }
    else
    {
        println!("Lag");
    }

    number_of_siblings;

}

pub fn age_of_siblings()
{
    let number_of_siblings = sibling_count();
    let mut age_of_siblings: Vec<u8> = Vec::with_capacity(number_of_siblings as usize);

    for x in number_of_siblings..number_of_siblings.len()
    {
        println!("Enter the age of sibling {:?}", x);

    }
}
fn main()
{
println!("Welcome, dear client!");
println!("What is your name?");
client_name();

}
