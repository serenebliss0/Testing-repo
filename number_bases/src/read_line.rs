pub fn read_line_as_i64() -> i64 
{
    let mut input = String::new();
    
    if let Err(e) = std::io::stdin().read_line(&mut input) 
    {
        println!("Oops reading input: {}", e);
        return 0; // fallback on error
    }

    match input.trim().parse::<i64>() {
        Ok(num) => num,
        Err(e) => {
            println!("Oops parsing input: {}", e);
            0 // fallback on parse error
        }
    }
}

pub fn read_line() -> String
{
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_string(),
        Err(lag) => {
            println!("Oops: {}", lag);
            return String::new()
        }
    }
}