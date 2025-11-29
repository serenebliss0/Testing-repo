use std::io;
fn main()
{
    //up to this point we've used this to collect user input

    let mut input = String::new();
    //std::io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("But you might've noticed, .expect will still cause the code to break");
    println!("So how can we avoid that issue?");

    //surprisingly with match statements!
    
    //the statement in line 6 returns a result with this format:
    //Result<usize, std::io::Error>
    //the values between <> are actually placeholders!!!!

    io::stdin().read_line(&mut input);

    let parsed = input.trim().parse::<i32>(); // Result<i32, ParseIntError>

    let num = match parsed {
        Ok(n) => n,  // success: bind the number to `n` and return it
        Err(e) => {
            println!("Oops! Not a number! Error: {}", e);
            return;   // exit the program early, or loop back for another input
        }
    };
    
    //now watch this, if you try
   // parsed + 1; //and the user types in "hello", it will panic without the match statement

  

    println!("There are three ways (from what i know 😅)");
    println!("match statements, like the one you see above this line");

    //if you were to remove that match statement, rust would panic and display this:

    /*Sorry bro: ParseIntError { kind: InvalidDigit }
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    error: process didn't exit successfully: `target\debug\learning_more_complicated_rust.exe` (exit code: 101)
    */
    
    println!("Anyways, thats just the first method! ");
    println!("The second is using if let...");

    let mut input_2 = String::new();

    if let Err(e) = std::io::stdin().read_line(&mut input_2)
    {
        println!("Oops! Something went wrong!!! {}", e);
    }
    else
    {
        println!("Input read successfully!");
    }

    println!("The third was is actually using the parse method!");

    let input_3 = String::new();


    let number:u32 = match input_3.trim().parse() {
        Ok(number) => number,
        Err(e) => 
        {
            println!("Invalid digit bro");
            return;
        }
    };

    //here, parse return a result <T, ParseIntError>

    }
