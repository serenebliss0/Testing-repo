use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Read;
use std::io::Write;
use std::str::FromStr;

pub trait Readable: Sized {
    fn read() -> Self;
}

impl<T> Readable for T
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    fn read() -> Self {
        'retry: loop {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                //for when no input is recieved
                Ok(0) => {
                    println!("No input received! Try again");
                    continue;
                }
                //do nothing for normal input
                Ok(_) => {}
                //return std::io::Error
                Err(lagg) => {
                    println!("Oops! {:?}", lagg);
                    continue;
                }
            }

            match input.trim().parse::<T>() {
                Ok(value) => return value,
                Err(lag) => println!("Oops! {:?}.", lag),
            }
        }
    }
}

pub fn create_file(filename_to_create: &str, overwrite: bool) -> Result<(), io::Error> {
    let _file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(overwrite)
        .open(filename_to_create)?;

    Ok(())
}

pub fn write_data(
    file_name: &str,
    data_to_write: &str,
    append: bool,
    overwrite: bool,
) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new()
        .append(append)
        .write(overwrite)
        .open("clients.serenity")?;

    let cleaned_output = format!("{}\n", data_to_write);
    file.write_all(cleaned_output.as_bytes())?;
    eprintln!("submitted successfully");

    Ok(())
}

pub fn read_data(file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(file_name)?;

    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    // Convert to String safely
    let contents = String::from_utf8_lossy(&bytes);

    println!("{}", contents);

    Ok(())
}
