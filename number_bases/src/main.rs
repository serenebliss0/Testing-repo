use core::num;
use std::io;
pub mod read_line;
use read_line::read_line_as_i64;
use read_line::read_line;

pub fn convert_to_bin(num: i64) 
{
    let binary = format!("{:b}", num);
    let reversed: String = binary.chars().rev().collect(); // collect iterator into String

    println!("Binary: {}", binary);       // normal binary
    println!("Reversed binary: {}", reversed); // reversed binary
}


fn is_binary_string(s: &str) -> bool 
{
    s.chars().all(|c| c == '0' || c == '1')
}


pub fn convert_to_dec(mut num: i64) -> i64
{
    let mut decimal = 0;
    let mut base = 1;

    while num > 0
    {
        let last_digit = num % 10;
        if last_digit > 1 
        {
            println!("Not a valid binary number!");
            return 0;
        }
        decimal += last_digit * base;
        base *= 2;
        num /= 10;
    }

    println!("The decimal value is {}", decimal);

    decimal
}


pub fn convert_to_oct(num:i64)
{
    let octal = format!("{:o}", num);
    println!("The octal value is {}", octal);
}

pub fn convert_to_hex(num:i64)
{
    let hexadecimal = format!("0x{:X}", num);
    println!("The hexadecimal value is {}", hexadecimal);
}

fn main()
{
    println!("Konnichiwa");

    println!("Which number base are you converting from?");
    let starting_base = read_line_as_i64();

    println!("Enter a number to convert:");

    let number_str = read_line(); // keep as String
    let num_str_slice = &number_str;

   if starting_base == 2 {
    let decimal = i64::from_str_radix(num_str_slice, 2).expect("Invalid binary number");
    println!("Decimal: {}", decimal);
    convert_to_bin(decimal);
    convert_to_hex(decimal);
    convert_to_oct(decimal);
}

if starting_base == 8 {
    let decimal = i64::from_str_radix(num_str_slice, 8).expect("Invalid octal number");
    println!("Decimal: {}", decimal);
    convert_to_bin(decimal);
    convert_to_hex(decimal);
    convert_to_oct(decimal);
}

if starting_base == 10 {
    let decimal = i64::from_str_radix(num_str_slice, 10).expect("Invalid decimal number");
    println!("Decimal: {}", decimal);
    convert_to_bin(decimal);
    convert_to_hex(decimal);
    convert_to_oct(decimal);
}

if starting_base == 16 {
    let decimal = i64::from_str_radix(num_str_slice, 16).expect("Invalid hexadecimal number");
    println!("Decimal: {}", decimal);
    convert_to_bin(decimal);
    convert_to_oct(decimal);
    convert_to_hex(decimal);
}

}