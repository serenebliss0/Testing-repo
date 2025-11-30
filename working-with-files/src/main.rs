use std::fs;
use std::fs::File; 
use std::fs::OpenOptions; //append some good text
use std::io::Write; //this has println's sister writeln!
use std::io;
use std::path::Path; //imma use this to check if file already exists
use std::process::Command; //now imma use this to run some commandsss

pub mod csv;
pub mod docx;

fn main()
{
    //fs::create_dir("test_folder").unwrap(); //create a good ol' folder
    //p.s. if the file already exists it ain't gonna run

    fs::create_dir_all("a/b/c").unwrap(); //creates nested folders
    fs::create_dir_all("serenity/bliss/user0/memories/random/lol/sigh/long/ahh/directory/name");

    //now lets create some txtsss

    fs::write("notes.txt", "Hello Serenitations").unwrap();
    
    fs::write("rm.serenity", "remove meeeeeeeeee" ).unwrap();

   // File::create("my-extension.serenity").unwrap(); 
    //you can create a file with any extension like this (although i can't guarantee they'll work in other programs!)

    let content = fs::read_to_string("my-extension.serenity").unwrap();
    println!("{}", content);

    println!("Type in some text!");

    let mut text_to_append = String::new();
    io::stdin().read_line(&mut text_to_append).expect("Failed to read line");

    let mut file = OpenOptions::new()
                .append(true)
                .open("my-extension.serenity")
                .unwrap();

    
    //>write!(file, "\n{}", text_to_append);

    //remember that append continues from the last character in the file
    //so you must use \n or else no new lines are created!!!

    //to remove a file

    fs::remove_file("rm.serenity").unwrap();

    //to remove a folder

    //fs::remove_dir("serenity").unwrap();

    //if the folder isn't empty you're gonna need some good recursion
    //but don't worry, rust does that for you (no -r needed)

    fs::remove_dir_all("serenity").unwrap();

    //ps if the dir isn't empty and you dont use remove_dir_all rust is gonna scream at you : )


    if Path::new("my-extension.serenity").exists()
    {
        println!("Unfortunately, the file already exists");
        println!("Would you like me to eliminate it?");
    }
    else
    {
        println!("Lucky you. This file hasn't been born yet!");
    }

    //reading directory contents
    for entry in fs::read_dir(".").unwrap()
    {
        let entry = entry.unwrap();
        println!("{:?}", entry.path())
    }


    //now lets run some external commanndddssssss

    let output = Command::new("ping")
                            .arg("google.com")
                            .output()
                            .expect("Failed to execute command");

    println!("{}", String::from_utf8_lossy(&output.stdout));

    //lets copy move and rename

    //copying

    fs::copy("my-extension.serenity", "serenitations.serenity").unwrap();
    fs::copy("notes.txt", "noted_bak.txt").unwrap();

    //rename or move (ren is the same as mv on windows)

    fs::rename("notes.txt", "notessssss").unwrap();

    //you can also read files except random txts and .serenity files

    //lets read a csv, raw bytes, even work with images
    
    let bytes = std::fs::read("pikachu.jpeg").unwrap();
    //this gives a Vec<u8> of raw bytes!

    println!("{:?}", bytes);

    //here's something cool i learned
    //if you typed in the bytes that you collected you can actually RECREATE the image and output it!!!!

    fs::write("test.jpg", bytes).unwrap();

    println!("Would you like to read or write a csv file?");
    
    let mut option = String::new();
    io::stdin().read_line(&mut option).expect("Failed to read line");
    let option:u8 = option.trim().parse().expect("Failed to read line");

    match option
    {
    1 => csv::read_csv(),
    2 => csv::write_csv(),
    _ => {
        println!("Choose a valid option");
        return;
    }
    };

    docx::make_docx();
}