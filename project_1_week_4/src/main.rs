use std::io;

fn main() 
{
    let mut input_1 = String::new();
    let mut input_2 = String::new();
    let mut input_3 = String::new();

    println!("Konnichiwa user\n Welcome to my Compound interest calc!\nEnter your Principal to begin");
    io::stdin().read_line(&mut input_1).expect("Enter a valid value");
    let mortgage_loan:f64 = input_1.trim().parse().expect("Enter a real number bruh, or else ill get angry");

    println!("Great! You successfully entered your principal. Now hand over your rate");
    io::stdin().read_line(&mut input_2).expect("Enter a good value bruh");
    let rate:f64 = input_2.trim().parse().expect("Bruh I gave you one chance already. dont make me mad");

    println!("Greattt. Now I only need the number of yearss");
    io::stdin().read_line(&mut input_3).expect("Bruh just import a good value!!!");
    let number_of_years = input_3.trim().parse().expect("That's your last chance. Your pc will suffer now");
 
            let amount:f64 = mortgage_loan * (1.00 + (rate /100.00 )).powf(number_of_years);
    let compound_interest:f64 = amount - mortgage_loan;
 
    println!("The compound interest is {:.4}", compound_interest);

}