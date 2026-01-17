#[warn(unused_mut)]
#[warn(unused_imports)]

use core::prelude;
use std::fmt::format;

//modules for taking user input
pub mod read_line;
use read_line::read_line;
use read_line::read_line_as_u8;

//modules for handling file management
pub mod create_file;
use create_file::makefile;
use create_file::write_data;

struct Client
{
    full_name:String,
    age: u8,
    number_of_siblings: u8,
}

#[derive(Debug)]
struct Sibling
{
    sibling_age: u8,
    marital_status: String,
    waec_status: u8,
    occupation: String,
    secondary_school: String,
    year_level: u8,
}

pub fn sibling_logic(sibling_count:u8)
{
    let mut marital_status:String = "null".to_string();
    let mut secondary_school:String = "null".to_string();
    let mut  year_level:u8 = 0;
    let mut sibling_age:u8 = 0;
    let mut occupation:String = "null".to_string();
    let mut university_course:String = "null".to_string();
    let mut workplace:String = "null".to_string();
    let mut waec_status:u8 = 0;

    let mut  Siblings = Vec::<Sibling>::new();


    for i in 1..=sibling_count
        {
            println!("How old is sibling {} ?", i);
            sibling_age = read_line_as_u8();

            if sibling_age >= 18
            {
                println!("Is sibling {} married, or single?", i);
                println!("1. Married");
                println!("2. Single");

                let marriage_user_choice = read_line_as_u8();

                match marriage_user_choice
                {
                    1 => {
                        println!("Sibling {} is married", i);
                        marital_status = "Married".to_string()
                    },
                    2 => {
                        println!("Sibling {} is single", i);
                        marital_status = "Single".to_string()
                    },
                    _ => println!("Please choose a valid option")
                }

                if marital_status == "Single"
                {
                    println!("Is sibling {} a university student or a worker?", i);
                    println!("Type student, worker, or unemployed");

                    occupation = read_line().to_lowercase();

                    match occupation.as_str()
                    {
                        "student" => {
                            println!("What university course is sibling {} enrolled in ", i);
                            university_course = read_line();
                        }
                        "worker" => {
                            println!("Where does sibling {} work?", i);
                            workplace = read_line();
                        }
                        _ => {
                            occupation = "unemployed".to_string();
                        }
                    }
                }
            }

            if sibling_age < 18
            {
                println!("Does sibling {} have their WAEC results?", i);
                println!("1. Yes, they have their results.\n2. No, they don't have their waec results");
                waec_status = read_line_as_u8();

               
                match waec_status
                {
                    1 => {
                        println!("What secondary school did sibling {} attend?", i);
                        secondary_school = read_line();
                    },
                    2 => {
                        println!("What year level is sibling {}", i);
                        year_level = read_line_as_u8();
            
                    },
                    _ => {println!("Enter a valid option")}
                }
            }
            
            let sibling = Sibling {
                sibling_age: sibling_age,
                marital_status: marital_status.clone(),
                waec_status: waec_status,
                occupation: occupation.clone(),
                secondary_school: secondary_school.clone(),
                year_level: year_level
                };
        Siblings.push(sibling);

        }

        println!("Your siblings' information are:");
        
    for s in &Siblings 
    {

    println!(
        "{} years old, {}, occupation: {}, secondary_school: {}, year_level: {}, WAEC: {}",
        s.sibling_age, s.marital_status, s.occupation, s.secondary_school, s.year_level, s.waec_status
    );

    let sibling_info_compiled = format!(
        "\n[Sibling Information:\n({} years old, \nMarital Status: {}, \nOccupation: {}, \nSecondary School: {}, \nYear Level: {}, \nWAEC: {})\n]",
        s.sibling_age,
        s.marital_status,
        s.occupation,
        s.secondary_school,
        s.year_level,
        s.waec_status
    );
    

    write_data(sibling_info_compiled);
    
    }
}

pub fn record_siblings(sibling_count:u8)
{
    if sibling_count == 0
    {
        println!("Your application has been collected successfully!");
    }
    else if sibling_count >= 1
    {
        sibling_logic(sibling_count);
    }
    else 
    {
        println!("Invalid value. Try again");
    }
}


fn main()
{
    makefile();
    let mut siblings = Vec::<Sibling>::new();

    println!("Welcome client! What is your full name?");
    let mut full_name = read_line();

    println!("How old are you?");
    let mut age = read_line_as_u8();
    println!("How many siblings do you have?");

    let mut sibling_count = read_line_as_u8();
    
    let client = Client
    {
        full_name: full_name.clone(),
        age: age,
        number_of_siblings: sibling_count,
    };

    let full_client_info = format!("
    Client: [ Full name: {}, Age: {}, Number of siblings: {}]", full_name.clone(), age, &sibling_count);

    write_data(full_client_info);
    record_siblings(sibling_count);

}