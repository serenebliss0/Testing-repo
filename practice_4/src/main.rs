use std::io;

fn main()
{
    println!("Student info Management System");

    //input name
    println!("\n Please enter your name!");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Please enter a valid name");

    println!("Your name is {}", name);

    println!("Enter your age");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Enter a valid age");

    let age:u32 = age.trim().parse().expect("Enter a vaild age bro");
    
    if age < 18 || age > 45
    {
        println!("Sorry you are not eligible for this activity");
    }
    else
    {
        return;
    }

    match age
    {
        0..=10 => println!("Bro what are you even doing here?"),
        11..=15 => println!("You still aint old enough"),
        16..=18 => println!("Now we're talking"),
        _ => println!("You have been banned from this service"),
    }

    while age < 10
    {
        println!("You are {} years old", age);
        println!("Go and sleep");

        if age ==10
        {
            println!("You have been excused!");
            break;
        }
    }


}