use core::prelude;
use std::io;

//Quadratic Equation solver

pub fn solve_quadratics()
{
    loop{

    //declaring variables to store coefficients
    let mut a_input = String::new();
    let mut b_input = String::new();
    let mut c_input = String::new();

    //taking input from user
    println!("Enter coefficient a:");
    io::stdin().read_line(&mut a_input).expect("Failed to read line");
    let a: f64 = match a_input.trim().parse() {
        Ok(a) => a,
        Err(e) => {
            println!("{}",e);
            solve_quadratics();
            return;
        }
    };
    
    if a == 0.00
    {
        println!("There's obviously no way a is 0.\nTry again!");
        solve_quadratics();
    }
    println!("Enter coefficient b:");
    io::stdin().read_line(&mut b_input).expect("Failed to read line");
    let b: f64 = match b_input.trim().parse() {
        Ok(b) => b,
        Err(e) => {
            println!("{}", e);
            solve_quadratics();
            return;
        }

    };

    println!("Enter coefficient c:");
    io::stdin().read_line(&mut c_input).expect("Failed to read line");
    let c: f64 = match c_input.trim().parse() {
        Ok(c) => c,
        Err (e) => {
            println!("{}", e);
            solve_quadratics();
            return;
        }
    };

    println!("Confirm that a is {}, b is {} and c is {} by typing (e) and press enter!", a, b, c);
    let mut confirm_choice = String::new();
    io::stdin().read_line(&mut confirm_choice).expect("Failed to read line");
    let confirm_choice = confirm_choice.trim().to_lowercase();

    if confirm_choice != "e"
    {
        println!("Okay fine, feel free to type in your correct values now!");
        solve_quadratics();
    }


    //calculating discriminant
    let discriminant = b * b - 4.0 * a * c;

    //displays the quadratic equation and its roots based on the discriminant
    if b >0.0
    {
    println!("The expression is: {}x^2 + {}x + {} = 0", a, b, c);
    //prints out the expression in the form ax^2 + bx + c
    }
    else 
    {
        println!("The expression is {}x^2 {}x + {} = 0", a, b, c);
        //changes the expression to ax^2 -bx +c 
    }

    if discriminant > 0.0 
    {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("The two real and distinct roots are: {:.4} and {:.4}", root1, root2);
    } 
    
    else if discriminant == 0.0 
    {
        let root = -b / (2.0 * a);
        println!("One real root: {:.4}", root);
    } 
    
    else 
    {
        //complex roots
        let real_part =  (-b / (2.0 *a));
        let imaginary_part = ((-discriminant).sqrt()) / (2.0 * a);
        println!("There are no real roots!");
        println!("The complex roots of the equation are\n");
        println!("{:.2} + {:.2}i and {:.2} - {:.2}i",real_part,imaginary_part,real_part,imaginary_part);
    }

    println!("Do you want to find the roots of another equation?\nChoose y or n");

    let mut input_1 = String::new();
    io::stdin().read_line(&mut input_1).expect("Enter y or n");
    let mut  option_1:String = input_1.trim().parse().expect("Enter y or n");

    //allows the user to input another equation as many times as they want
    if option_1 == "y"
    {
       solve_roots();
    }
    else
     {
        {
            println!("What would you like to do?");
            println!("1. Find the roots of an equation in the form ax^2 + bx + c\n2. Find the equation with two roots alpha and beta\n3. Find the equation with two complex roots\n4. Exit the program");
    
            let mut option = String::new();
            io::stdin().read_line(&mut option).expect("Failed to read line");
            let option:u8 = option.trim().parse().expect("Type in a valid number from the list");
        
            match (option)
            {
                1 => solve_quadratics(),
                2 => solve_roots(),
                3 => complex_roots(),
                4 => {
                    println!("Goodbye! Come back some other time!");
                    break;
                },
                _ => break,
            };
            
    
        }
    
   
}
    }
}

//this function will be very useful when trying convert a.b into a/b
fn to_fraction(x: f64) -> (i64, i64) {
    let s = x.to_string();

    if let Some(pos) = s.find('.')
     {
        let decimals = (s.len() - pos - 1) as u32;
        let denom = 10_i64.pow(decimals);
        let numer = (x * denom as f64).round() as i64;
        (numer, denom)
    } else {
        // whole number
        (x as i64, 1)
    }
}

//ill use this to find an equation from two roots!
pub fn solve_roots()
{
    loop
    {
    println!("Okay, enter the two roots of the quadratic equation!");
    println!("What is root 1:");
        //just gathering some root inputs
    let mut root_1 = String::new();
    io::stdin().read_line(&mut root_1).expect("Failed to read line");
    let root_1:f64 = match root_1.trim().parse() {
        Ok(root_1) => root_1,
        Err(e) => {
            println!("{}", e);
            solve_roots();
            return;
        }
    };

    println!("What is root 2:");
    let mut root_2 = String::new();
    io::stdin().read_line(&mut root_2).expect("Failed to read line");
    let root_2:f64 = match root_2.trim().parse() {
        Ok(root_2) => root_2,
        Err(e) => {
            println!("{}", e);
            solve_roots();
            return;
        }
    };

    let sum_of_roots = root_1 + root_2;
    let product_of_roots = root_1 * root_2;

        //stores the coefficients in the form a/b using the previously made to_fractions functions
        let (b_num, b_den) = to_fraction(sum_of_roots);
        let (c_num, c_den) = to_fraction(product_of_roots);
        
        // find LCM of denominators
        fn gcd(a: i64, b: i64) -> i64
         {
         if b == 0 
         { a.abs() 
        }
          else 
          { gcd(b, a % b)
         }
         }

         fn lcm(a: i64, b: i64) -> i64 {
           (a * b) / gcd(a, b)
         }

let l = lcm(b_den, c_den);

// scale all coefficients
let a = l;                               // because a = 1 originally
let b = -b_num * (l / b_den);
let c =  c_num * (l / c_den);

    println!("The equation with roots {} and {} is", root_1, root_2);
    println!("x^2 -{}x + {} = 0", sum_of_roots, product_of_roots);
    println!("This is also written as: {}x^2 + {}x + {} = 0", a, b, c);

println!("Do you want to find the equation with another set of roots? y/n");

let mut option = String::new();
io::stdin().read_line(&mut option).expect("Failed to read line");
let option = option.trim();

if option == "y"
{
   solve_roots();
}
else {
    {
        println!("What would you like to do?");
        println!("1. Find the roots of an equation in the form ax^2 + bx + c\n2. Find the equation with two roots alpha and beta\n3. Find the equation with two complex roots\n4. Exit the program");

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read line");
        let option:u8 = option.trim().parse().expect("Type in a valid number from the list");
    
        match (option)
        {
            1 => solve_quadratics(),
            2 => solve_roots(),
            3 => complex_roots(),
            4 => {
                println!("Goodbye! Come back some other time!");
                break;
            },
            _ => break,
        };
        

    }
}

}
}

pub fn complex_roots()
{
    println!("This function hasn't been implemented yet\nCheck back later : )");
}
fn main()
{
    println!("Welcome to macro quad solve (rust edition)!");
    println!("Please choose an option");
    println!("1. Find the roots of an equation in the form ax^2 + bx + c\n2. Find the equation with two roots alpha and beta\n3. Find the equation with two complex roots");

    let mut option = String::new();
    io::stdin().read_line(&mut option).expect("Failed to read line");
    let option:u8 = match option.trim().parse() {
        Ok(option) => option,
        Err(e) => {
            println!("{}", e);
            main();
            return;
        }
    };

    match (option)
    {
        1 => solve_quadratics(),
        2 => solve_roots(),
        3 => complex_roots(),
        _ => {
            println!("Enter a valid option bro");
            main();
        },
    };
    }
