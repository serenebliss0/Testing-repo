use std::io;
use std::fs;
fn main()
{
    //a simple student management system

    println!("Welcome to my simple SIMS!\nWhat would you like to do?");
        println!("1. Add a student\n2. Delete a student\n3. View all students\n4. Assign a grade to a student");
   //storing names inside a vector
    let mut student_names:Vec<String> = vec!["semire".to_string()];
    let mut grades:Vec<String> = vec![];

    //collecting user input
    let mut input_1 = String::new();
    io::stdin().read_line(&mut input_1).expect("Enter a valid argument");
    let option_1:String = input_1.trim().to_string();
    //decide which operation to carry out

    if option_1 == "1" {
        println!("How many students will you add?");
        let mut  input_2 = String::new();
        io::stdin().read_line(&mut input_2).expect("Enter a valid option");
        let option_2:u8 = input_2.trim().parse().expect("Enter a valid number");
        //option 2 is the number of students to be added

        let mut x = 0;
        while x < option_2
        {
            x += 1;
            println!("Enter the name of student {}:", x);
            let mut name = String::new();
            io::stdin().read_line(&mut name).expect("Enter a name");
            let student_name = name.trim().to_lowercase();
            student_names.push(student_name.clone());
            println!("The student name you entered is {:?}", student_name);
            println!("The list of students in the class are{:?}", student_names);
        }
    } else if option_1 == "2" {
        println!("Enter the name of the student to remove:");
        let mut name_to_remove = String::new();
        io::stdin().read_line(&mut name_to_remove).expect("Enter a name");
        let name_to_remove = name_to_remove.trim().to_lowercase();

        if student_names.contains(&name_to_remove)
        {
            student_names.retain(|n| n != &name_to_remove);
            println!("✅ {} removed successfully!", name_to_remove);
        }
        else
        {
            println!("Student '{}' not found.", name_to_remove);
        }
        println!("Updated student list: {:?}", student_names);
    }
     else if option_1 == "3" {
        println!("Current students: {:?}", student_names);
    } 
    else if option_1 == "4" {
        
        println!("Enter the name of the student:");
        let mut name_to_grade = String::new();
        io::stdin().read_line(&mut name_to_grade).expect("Enter a name");
        let name_to_grade = name_to_grade.trim().to_lowercase();

        if student_names.contains(&name_to_grade)
        {
            println!("Enter the grade for {}:", name_to_grade);
            let mut grade = String::new();
            io::stdin().read_line(&mut grade).expect("Enter a grade");
            let grade = grade.trim().to_string();


            println!("Grade is {} ", grade);
         } 
         else{
            println!("Student '{}' not found.", name_to_grade);
         }

}

}

