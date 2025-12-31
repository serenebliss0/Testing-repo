use std::fmt::format;
use std::io::Write;
use std::fs::OpenOptions;

pub fn makefile() {
    OpenOptions::new()
        .write(true)
        .create(true)   // creates if missing
        .open("clients.serenity")
        .expect("Failed to create/open clients file");
}

pub fn write_data(data_to_write:String)
{
    let mut client_file = OpenOptions::new().append(true).open("clients.serenity")
    .expect("Unable to open client records");

    let cleaned_output = format!("{}\n", data_to_write);
    client_file.write_all(cleaned_output.as_bytes()).expect("Failed to write");
    println!("Application submitted successfully");
}
