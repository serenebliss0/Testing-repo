use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;
use std::env;
use std::{thread, time};
use clap::Parser;

fn main()
{
    println!("Welcome to the rust edition of AutoFileCreate");
    println!("This program will help you create all the rust source files you need for your project");

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
        fs::write("config.serenity", &mainPath); //write to config file

    }


    //directory path
    println!("What week is this currently?");

    let mut week_index = String::new();
    io::stdin().read_line(&mut week_index).expect("Failed to read line");
    let week_index:u8 = match week_index.trim().parse() {
        Ok(week_index) => week_index,
        Err(lag) => {
            println!("{}", lag);
            return;
        }
    };

    let directoryPath = format!("{}/week_{}", mainPath, week_index );
    fs::create_dir_all(&directoryPath);

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

    

    /*  if directoryPath == is_none()
    {
        println!("Bruh I can't just make files out of thin air");
        println!("Try again");
        return;
    }
*/



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
            let output = Command::new("cargo")
                                    .arg("new")
                                    .arg(&current_practice)
                                    .output()
                                    .expect("Failed to write files");
        }
        println!("All practice directories have been created");
        thread::sleep(time::Duration::from_secs(2));
    }

    println!("Now how many project files do you desire?");

    let mut NumOfProjectFiles = String::new();
    io::stdin().read_line(&mut NumOfProjectFiles).expect("Failed to read line");
    let NumOfProjectFiles:u8 = match NumOfProjectFiles.trim().parse() {
        Ok(NumOfProjectFiles) => NumOfProjectFiles,
        Err(e) => 
        {
            println!("{}", e);
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
            let output = Command::new("cargo")  
                                    .arg("new")
                                    .arg(&current_project)
                                    .output()
                                    .expect("Failed to write files");
        }
        println!("All project files have been created!");
    }

}