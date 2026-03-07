use std::collections::HashMap;
use std::io;
//reminds me of using namespaces in C#

fn main() {
    print!("Konnichiwa, Sekai!");

    print!("\nThis is a test \n");

    let mut x = 5;
    x = 10;

    println!("The value of x is: {}", x);

    let my_number = 5; //integer
    let my_double = 5.99; //floating point
    let my_letter = 'd'; //character
    let my_boolean = true; //boolean
    let my_text = "Hello"; //string

    let my_num:i32 = 5; //32 bit integer
    let my_num2:f64 = 5.99; //64 bit floating point
    let my_letter2:char = 'd'; //character
    let my_boolean2:bool = true; //boolean
    let my_text2:&str = "Hello"; //string

    const PI:f64 = 3.141592; //stores pi as a constant value

    println!("The value of pi is: {}", PI);

    if my_num <10 && my_num2 >5.00
    {
        println!("IF statements work just like in C#. No surprises here.");
    }

    let is_programming_fun = true;
    let is_math_hard = false;
    let is_rust_fun = true;
    let is_rust_like_csharp = true;

    println!("Is programming fun? {}", is_programming_fun);
    

    let age = 20;
    let can_vote = age >= 18;
    println!("Can you vote? {}", can_vote);

    let is_logged_in = true;
    let is_first_time_user = false;

    if is_logged_in 
    {
        println!("Welcome back!");
    }
    else if  is_logged_in && is_first_time_user
    {
        println!("Welcome to your dashboard!");
    }
    else
    {
        println!("Please log in to continue.");
    }

    match age {
        0..=12 => println!("You are a child."),
        13..=19 => println!("You are a teenager."),
        20..=64 => println!("You are an adult."),
        _ => println!("You are a senior citizen."),
    }

    let time = 20;
    let greeting = if time < 18 {"Good day"} else {"Good evening"};
    println!("{}", greeting);

    let day = 4;

    match day 
    {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day"),
    }

    match day {
        1 | 2 | 3 | 4 | 5 => println!("Weekday"),
        6 | 7 => println!("Weekend"),
        _ => println!("Invalid day"),
    }

    println!("{}", day);
    
    loop 
    {
        println!("This will run forever.");
        break;
    }

    let mut count = 1;

    loop 
    {
    println!("Hello world!");

    if count == 3 
    {
        break;
    }

    count += 1;
    }


    while count <= 5
    {
        println!("Count is {}", count);
        count += 1;

        if count == 4
        {
            count += 1;
            continue;
        }
    }

    for i in 1..6 
    {
        println!("i is {}", i);
    }

    //or alternatively if you need to add the number 6 as well

    for i in 1..=6 
    {
        println!("\n i is {}", i);
    }

    for i in 1..=10 {
  if i == 3 {
    continue; // skip 3
  }
  if i == 5 {
    break; // stop before printing 5
  }
  println!("i is: {}", i);
}

fn testing_function() {
    println!("This is a test function.");
}

testing_function();

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

    greet("Serenity");

    fn add(a: i32, b: i32) -> i32
    {
        return a + b;
    }

    let sum = add(3,4);
    println!("The sum is {}", sum);

fn myFunction() 
{
    let message = "Hello!";
    println!("{}", message)
}

let greeting2: &str = "こんにちわ、世界！"; 
println!("{}", greeting2);

let text1 = String::from("Hello, ");
let text2 = "Hello world".to_string();

let mut greeting3 = String::from("Konnichiwa, Sekai");
greeting3.push_str("!");

println!("{}", greeting3);

let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = String::from("What a beautiful day!");
let result = format!("{}{} {}", s1, s2, s3);
println!("{}", result);

let result2 = s1 + &s2 + &s3; 
println!("{}", result2);

let name2 = String::from("Serenity");
println!("Length: {}", name2.len());

let a1 = String::from("Hello");
let b1 = a1;

println!("{}", b1);
/*println!("{}", a1); // This will cause an error because a1 has been moved to b1*/

let a2 = 5;
let b2 =a2;

println!("{}", a2);
println!("{}", b2);

let a3 = String::from("Hello");
let b3 = a3.clone();

println!("{}", a3);
println!("{}", b3);

let a4 = String::from("Hello");
let b4 = &a4;

println!("a is {}", a4);
println!("b is {}", b4);

let mut name = String::from("Serenity");
let  name_ref = &mut name;
name_ref.push_str(" is learning Rust");
println!("{}", name_ref);

let mut numbers = [1, 2, 3, 4, 5];

println!("First number: {}", numbers[0]);

numbers[0] = 10;
println!("First number after change: {}", numbers[0]);
println!("This array had {} elements.", numbers.len());

let fruits = ["Apple", "Banana", "Cherry"];

for fruit in fruits
{
    println!("I like {}", fruit);
}

println!("All the fruits in the basket are {:?}", fruits);

let mut  fruits = vec!["Apple", "Banana", "Cherry"];
println!("First fruit: {}", fruits[0]);

fruits[0] = "Orange";
println!("First fruit after change: {}", fruits[0]);
println!("This vector has {} elements.", fruits.len());

let person = ("Serenity", 25, true);
println!("Name: {}", person.0);
println!("Age: {}", person.1);
println!("Is student: {}", person.2);

//unpacking a tuple

let (name, age, is_student) = person;

println!("Name is: {}", name);
println!("Age is: {}", age);
println!("Is student: {}", is_student);

fn get_user() -> (String, i32, bool)
{
    (String ::from("Serenity"), 25, true)
}

let user = get_user();
println!("User is {} ({} years old). Student: {}", user.0, user.1, user.2);

let mut capitalCities = HashMap::new();
//created a new hash map

//Adding keys and values (Country, Cities)
capitalCities.insert("England", "London");
capitalCities.insert("Germany", "Berlin");
capitalCities.insert("Norway", "Oslo");
capitalCities.insert("USA", "Washington DC");
capitalCities.insert("Turkey", "Ankara");
capitalCities.insert("日本", "東京");

println!("{:?}", capitalCities);

//accessing values in a hash map

if let Some(city) = capitalCities.get("Japan")
{
    println!("The capital of Japan is {}", city);
}
else
{
    println!("City not found");
}

for (country,city) in &capitalCities
{
    println!("The capital of {} is {}", country, city);
}

struct person   
{
    name: String,
    age: u32,
    is_student: bool,
}

//creating an instance of the struct
let mut user = person{
    name: String::from("Serenity"),
    age: 25,
    is_student: true,
};
println!("Name: {}", user.name);
println!("Age: {}", user.age);
println!("Is student: {}", user.is_student);

//enums (enumerations)

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right, 
}

let my_direction = Direction::Up;
println!("We're going {:?}, {:?}, {:?} it's our moment", my_direction, my_direction, my_direction);

match my_direction 
{
    Direction::Up => println!("Gonna be gonna be Golden"),
    Direction::Down => println!("I guess we're going down"),
    Direction::Left => println!("We ain't going up or down now"),
    Direction::Right => println!("You're my soda pop"),
    _ => println!("We don't know where we're going"),
}

enum LoginStatus
{
    Success(String),
    Failure(String),
}

let status = LoginStatus::Success(String::from("Welcome back, Serenity!"));
let status2 = LoginStatus::Failure(String::from("Invalid username or password."));

match status 
{
    LoginStatus::Success(message) => println!("{}", message),
    LoginStatus::Failure(message) => println!("{}", message),
}

//Tom’s Obvious, Minimal Language (TOML) is a configuration file format used by Rust's package manager, Cargo, to manage project dependencies and settings.





}

