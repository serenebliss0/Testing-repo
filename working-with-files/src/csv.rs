use std::fs;
use std::error::Error; //looks like an error is about to happen, oh well?
use csv::Reader; //speaks for itself. its gonna help me read some csv's
use csv::Writer; //write some good csvssssss


pub fn read_csv() -> Result<(), Box<dyn Error>>
{
    
    let mut rdr = Reader::from_path("student_scores.csv")?;

    for result in rdr.records()
    {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}

pub fn write_csv() -> Result<(), Box<dyn Error>>
{
    let mut wtr = Writer::from_path("output.csv")?;
    wtr.write_record(["name", "age"])?;
    wtr.write_record(["Serenity", "16"])?;
    wtr.flush()?;

    Ok(())
}