use structopt::StructOpt;
use colored::*;
use semire_core::*;


#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Meow!")]
    //what does the cat say?
    message: String,

    #[structopt(short = "d", long = "dead")]
    //Make the cat appear dead
    dead: bool,

    #[structopt(short = "f", long = "file", parse(from_os_str))]
    catfile: Option<std::path::PathBuf>,
}

fn main() 
{
    let options = Options::from_args();
    let message = options.message;
    println!("{}", message);

    let eye = if options.dead {"x"} else {"o"}; 

    match &options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path)
                                        .expect("Could not read file");
            let cat_picture = cat_template.replace("{}", eye);
            println!("{}", &cat_picture);
        },
        None => {

        }
    }
        println!(r#"
     /\_/\  
    ( {}.{} ) 
     > ^ <
    "#, eye.red().bold(), eye.red().bold());
    
    if message.to_lowercase() == "woof"
    {
        eprintln!("Why on earth would you want a cat to bark?");
        //cargo run "woof" 1> stdout.txt 2 > stderr.txt
    }
}