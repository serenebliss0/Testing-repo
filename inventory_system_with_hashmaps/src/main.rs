use std::collections::HashMap;
use std::io;

fn main()
{
    let mut inventory = HashMap::new();

inventory.insert("l", 550000);
inventory.insert("m", 120000);
inventory.insert("k", 15000 );
inventory.insert("h", 25000);


println!("Welcome user!");
println!("Choose an item code from the menu below");

println!("{:?}", inventory);
let mut option = String::new();
io::stdin().read_line(&mut option).expect("Failed to read line");
let option = option.trim().to_lowercase(); // keep as String

let mut user_choice:(String, &i32) = ("".to_string(), &0);

if let Some(price) = inventory.get(option.as_str())
 {
    user_choice = (option, price);
} 
else
 {
    println!("Item not found");
}


let (item_code, item_price) = user_choice;

let item_name = match item_code.as_str() {
    "l" => "laptop".to_string(),
    "m" => "monitor".to_string(),
    "k" => "keyboard".to_string(),
    "h" => "headset".to_string(),
    other => format!("Unknown item code: {}", other),
};




}