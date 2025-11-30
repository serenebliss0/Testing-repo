use std::io;

pub fn option_redirector(option: u8, angle: f64) -> f64 {
    match option {
        1 => sine_as_degrees(angle),
        2 => cosine_as_degrees(angle),
        3 => tangent_as_degrees(angle),
        _ => {
            println!("Invalid option!");
            return f64::NAN; // return something, ANYTHING, it's required
        }
    }
}

pub fn sine_as_degrees(angle: f64) -> f64 {
    angle.to_radians().sin()
}

pub fn cosine_as_degrees(angle: f64) -> f64 {
    angle.to_radians().cos()
}

pub fn tangent_as_degrees(angle: f64) -> f64 {
    angle.to_radians().tan()
}

fn main() {
    println!("Something will go here one day!");

    println!("Which operation would you like to perform?");
    println!("1. Sine");
    println!("2. Cosine");
    println!("3. Tangent");

    // get option
    let mut option = String::new();
    io::stdin().read_line(&mut option).expect("Failed to read option");

    let option: u8 = match option.trim().parse() {
        Ok(o) => o,
        Err(e) => {
            println!("Invalid option: {}", e);
            return;
        }
    };

    // get angle
    println!("Enter an angle!");

    let mut angle = String::new();
    io::stdin().read_line(&mut angle).expect("Failed to read angle");

    let angle: f64 = match angle.trim().parse() {
        Ok(a) => a,
        Err(e) => {
            println!("Invalid angle: {}", e);
            return;
        }
    };

    let result = option_redirector(option, angle);
    println!("Result: {}", result);
}
