use std::io;

pub fn convert_to_bin(num:i64)
{
    let binary = format!("{:b}", num);

    println!("The binary value is {}", binary);
    
}

pub fn convert_to_dec(num:i64)
{
    let decimal = format!("{:E}", num);
    println!("The decimal value is {}", decimal);
}

pub fn convert_to_oct(num:i64)
{
    let octal = format!("{:o}", num);
    println!("The octal value is {}", octal);
}

pub fn convert_to_hex(num:i64)
{
    let hexadecimal = format!("{:X}", num);
    println!("The hexadecimal value is {}", hexadecimal);
}

fn main()
{
    println!("Konnichiwa");
    println!("Enter a number to convert");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut number:i64 = match input.trim().parse() {
        Ok(number) => number,
        Err(e) => 
        {
            println!("Sorry bro, you typed in garbage. Try again");
            return;
        }
    };
    convert_to_bin(number);
    convert_to_dec(number);
    convert_to_oct(number);
    convert_to_hex(number);
}