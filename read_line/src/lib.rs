use std::str::FromStr;
use std::io;

pub trait Readable: Sized 
{
    fn read() -> Self;
}

impl<T> Readable for T
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    fn read() -> Self 
    {
        'retry: loop 
        {
            let mut input = String::new();
            match io::stdin().read_line(&mut input)
            {
                //for when no input is recieved
                Ok(0) => {
                    println!("No input received! Try again");
                    continue;
                }
                //do nothing for normal input
                Ok(_) => {},
                //return std::io::Error
                Err(lagg) =>
                {
                    println!("Oops! {:?}", lagg);
                    continue;
                }
            }

            match input.trim().parse::<T>() 
            {
                Ok(value) => return value,
                Err(lag) => println!("Oops! {:?}.", lag),
            }
        }
    }
}
