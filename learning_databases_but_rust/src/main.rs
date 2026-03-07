use rusqlite::*;
use std::io::*;
use std::fs::*;

pub fn AddStudent(conn: &Connection)
{
    println!("Hello");
}

pub fn ShowStudents(conn: &Connection) 
{
        let mut stmt = conn.prepare("SELECT id, name, age FROM users").expect("Prepare failed");
    
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, i32>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i32>(2)?,
            ))
        }).expect("Query failed");
    
        for row in rows {
            let (id, name, age) = row.expect("Row unwrap failed");
            println!("ID: {}, Name: {}, Age: {}", id, name, age);
        }
    }
    

fn main() 
 {
    let conn = match Connection::open("students.sqlite3") {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to open database: {}", e);
            return;
        }
    };
    println!("Database connected!");
    

    let _ = conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        age INTEGER
    )",
    [],
    );

    //back to good old user input now

    loop
    {
        println!("What would you like to do today!!");
        println!("1. Add a user\n2. List all users\n3. Exit");

        let mut option = String::new();
        std::io::stdin().read_line(&mut option).expect("Failed to read line");
        let option:u8 = match option.trim().parse() {
            Ok(option) => option,
            Err(e) => 
            {
                println!("{}", e);
                return;
            }
        };

        match option
        {
            1 => AddStudent(&conn),
            2 => ShowStudents(&conn),
            _ => println!("Failed to read line"),
        };
        
    }
}
