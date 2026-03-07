use std::io;

pub fn read_line() -> String
{
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_string(),
        Err(lag) => {
            println!("{}", lag);
            String::new()
        }
    }
}

pub fn do_binomial_two_terms(x:u64, y:u64, n:u64) -> u64
{
    let mut output:u64 =0;

    if n > 1
    {
    for index in 0..=n
    {
        let mut term = (combination(n, index) * (x.pow((n- index) as u32)) * (y.pow(index as u32)));
        println!("The {} term is {}", index, term);

        output+=term;
    }
    }
    else if n == 1
    {
        let mut term = x + y;
        return term
    }
    else
    {
        println!("n must be positive for the mode you selected! Try again");
        return 0
    }
    
    output

}

pub fn factorial(num:u64) -> u64
{
        (1..=num).product()
    
}

    pub fn combination(n: u64, r: u64) -> u64 
    {
        if r > n 
        { return 0; }

        let combination:u64 = factorial(n) / (factorial(r) * factorial(n - r));

        combination
    }
    

fn main()
{
    loop
    {
    //it's been a while since i've pushed something on this repo

    println!("Welcome user! Welcome to my new project!\nA binomial expansion calculator");
    println!("Start by choosing an option!");

    println!("1. Perform expansion with an equation in the form (a + b)^n ");
    println!("2. Perform expansion with an equation in the form (a +b +c)^n (coming soon)");
    println!("3. Perform expansion with an equation in the form (a +b)^-n (coming soon)");

    let user_choice:u8 = match read_line().parse() {
        Ok(user_choice) => user_choice,
        Err(lagging) => {
            println!("{}", lagging);
            return;
        }
    };

    match user_choice 
    {
        1 => collect_all_pos(),
        _ => attempt_unavailable()
    }

    println!("Would you like to run again?\nType `y` or `n`");

    let mut option = read_line();

    if option!="y"
    {
        println!("Bye, have a great time <3");
        break;
    }

    }
}

pub fn attempt_unavailable()
{
    println!("Sorry lol, that option isn't available at the moment!\nMaybe I'll add that later!");
}

pub fn collect_all_pos()
{

        println!("Enter an equation in the form (a+b)^n");
        println!("Enter a value for a:");

        let a_val:u64 = match read_line().parse() {
            Ok(a_val) => a_val,
            Err(lagger) => {
                println!("{}",lagger);
                return;
            }
        };

        println!("Enter a value for b:");

        let b_val:u64 = match read_line().parse() {
            Ok(b_val) => b_val,
            Err(lagger) => {
                println!("{}",lagger);
                return;
            }
        };

        println!("Enter a value for n:");

        let n_val:u64 = match read_line().parse() {
            Ok(n_val) => n_val,
            Err(lagger) => {
                println!("{}",lagger);
                return;
            }
        };

        println!("Your equation was ({} + {}) ^ {}", a_val, b_val, n_val);
        if a_val <= u64::MAX && b_val <=u64::MAX && n_val <= u64::MAX
        {
        let final_answer = do_binomial_two_terms(a_val, b_val, n_val);
        println!("After expansion, the final answer is {}", final_answer);
        }
        else
        {
            println!("Lol my calculator cannot handle values this crazy");
        }
    }
