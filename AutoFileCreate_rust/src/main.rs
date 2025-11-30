use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;
use std::env;
use std::{thread, time};
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    week: Option<u8>,  // --week 9
    #[arg(long)]
    config: bool,      // --config
}

fn main()
{
    let args = Args::parse();

    println!("Welcome to the rust edition of AutoFileCreate"); //semire was here : )
    println!("This program will help you create all the rust source files you need for your project");

    // If user wants to open config.serenity
    if args.config {
        if Path::new("config.serenity").exists() 
        {
            let config_contents = fs::read_to_string("config.serenity").expect("Couldn't read config.serenity");
            
            let output = Command::new("notepad")
            .arg("config.serenity")
            .output()
            .expect("Failed to open config.serenity");

        } 
        else 
        {
            println!("config.serenity doesn't exist yet. Creating an empty one...");
            fs::write("config.serenity", "").expect("Couldn't write new config");
        }
        return; // stop program here in config mode
    }

    // Load or create mainPath
    let mut mainPath = String::new();

    if Path::new("config.serenity").exists()
    {
        mainPath = fs::read_to_string("config.serenity").expect("Failed to read config.serenity");
    }
    else
    {
        println!("Type in the directory path of your COS101 folder");
        io::stdin().read_line(&mut mainPath).expect("Failed to read line");
        mainPath = mainPath.trim().to_string();
        fs::write("config.serenity", &mainPath).expect("Failed to write config file");
    }

    // WEEK selection (if --week 8 was passed, use it)
    let week_index:u8 = match args.week {
        Some(w) => w,
        None => {
            let mut temp = String::new();
            println!("What week is this currently?");
            io::stdin().read_line(&mut temp).expect("Failed to read line");
            match temp.trim().parse()
            {
                Ok(temp) => temp,
                Err(lag) => {
                    println!("{}", lag);
                    return;
                }
            }
        }
    };

    let directoryPath = format!("{}/week_{}", mainPath, week_index );
    fs::create_dir_all(&directoryPath).expect("Couldn't create directory");

    // Number of practice files
    println!("How many practice files do you want to create");

    let mut NumOfPracticeFiles = String::new();
    io::stdin().read_line(&mut NumOfPracticeFiles).expect("Failed to read line");
    let NumOfPracticeFiles:u8 = match NumOfPracticeFiles.trim().parse() {
        Ok(NumOfPracticeFiles) => NumOfPracticeFiles,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let path = Path::new(&directoryPath);
    env::set_current_dir(path).expect("Couldn't change directory");

    if NumOfPracticeFiles == 0
    {
        println!("No files to create\nGoodbyeeee!");
        return;
    }
    else
    {
        for i in 1..=NumOfPracticeFiles
        {
            let current_practice = format!("practice_{}", i);
            Command::new("cargo")
                .arg("new")
                .arg(&current_practice)
                .output()
                .expect("Failed to create practice");
        }
        println!("All practice directories have been created");
        thread::sleep(time::Duration::from_secs(2));
    }

    // Number of project files
    println!("Now how many project files do you desire?");

    let mut NumOfProjectFiles = String::new();
    io::stdin().read_line(&mut NumOfProjectFiles).expect("Failed to read line");
    let NumOfProjectFiles:u8 = match NumOfProjectFiles.trim().parse() {
        Ok(NumOfProjectFiles) => NumOfProjectFiles,
        Err(lag_again) => {
            println!("{}", lag_again);
            return;
        }
    };

    if NumOfProjectFiles == 0
    {
        println!("Oooh no projects this week...");
        return;
    }
    else
    {
        for i in 1..=NumOfProjectFiles
        {
            let current_project = format!("project_{}", i);
            Command::new("cargo")
                .arg("new")
                .arg(&current_project)
                .output()
                .expect("Failed to create project");
        }
        println!("All project files have been created!");
    }
}
