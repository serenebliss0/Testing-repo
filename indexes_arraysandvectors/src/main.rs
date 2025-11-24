use std::io;
use num::integer::gcd;
fn main()
{

    let arr = [10, 20, 30, 40];
    let index = 1;

    //get returns Option<&i32>

    if let Some(value) = arr.get(index)
    {
        println!("Value at index {} is {}", index, value);

    }
    else
    {
        println!("There is no value at the specified index!!!");
    }

    //this works exactly the same way we used the .get() method but now there's some error handling 
    //for when there's no value at the specified index!!

    //but what happens if we don't know the index?

    //this is from question 3 in CA 1 btw

    let items = vec!["laptop", "keyboard", "monitor", "mouse"];

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line!!!");
    let input = input.trim().to_lowercase();

    let index = items.iter().position(|&item| item == input);

    match index
    {
        Some(i) => println!("Found {} at index {}", input, i),
        None => println!("{} not found in this inventory!", input),
    }

    number_to_fraction();

}

pub fn number_to_fraction()
{
    //remember this issue that i faced from the quadratic solver?
    //finding the position where the decimal point is in a.b ?

    println!("Type in a floating point number!");
    let mut input_2 = String::new();
    io::stdin().read_line(&mut input_2).expect("Failed to read line!");
    let number:f64 = input_2.trim().parse().expect("Type in a floating point number!");
    let number_as_str = number.clone().to_string();

    if let Some(pos) = number_as_str.chars().position(|c| c == '.')
    {
        println!("A decimal point was found at {}", pos);
        let int_part = &number_as_str[..pos];
        let frac_part = &number_as_str[pos+1..];

        let numerator = format!("{}{}",int_part, frac_part);
        let denominator = 10u64.pow(frac_part.len() as u32);

        let divisor = gcd(numerator.parse::<u64>().unwrap(), denominator);

        let reduced_numerator = numerator.parse::<u64>().unwrap() / divisor;
        let reduced_denominator = denominator / divisor;

        println!("The number {} can actually be written as {}/{}", number, reduced_numerator, reduced_denominator);
    }
    else
    {
        println!("bruh I said floating point yet, you still typed in a whole number!");
    }



}