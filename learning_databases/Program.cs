using System;
using System.Runtime.ConstrainedExecution;
using dotenv.net;
using Npgsql;
//choose your libraries with caution!

//creating the student class
class student_tracker
{
    static void Main()
    {
        //remove this before committing!!!!
        DotEnv.Load();
        var connString = Environment.GetEnvironmentVariable("DB_CONNECTION");
        using (var conn = new NpgsqlConnection(connString))
        {
            conn.Open();
            System.Console.WriteLine("Connected Successfully");

            System.Console.WriteLine("Welcome to the advanced student management system");
            while (true)
            {
                System.Console.WriteLine("Please choose an option");
                System.Console.WriteLine("1. Add a student");
                System.Console.WriteLine("2. View all students");
                System.Console.WriteLine("3. Update student info");
                System.Console.WriteLine("4. Delete a student");
                System.Console.WriteLine("5. Search for a student");
                System.Console.WriteLine("6. Exit");

                string choice = Console.ReadLine();

                switch (choice)
                {
                    case "1": AddStudent(conn); break;
                    case "2": ShowStudents(conn); break;
                    case "3": UpdateStudent(conn); break;
                    //case "4": DeleteStudent(conn); break;
                    //case "5": SearchStudent(conn); break;
                    case "6": break;
                    default: System.Console.WriteLine("Choose from the list and try again"); return;

                }
            }


        }

        //adding students
        static void AddStudent(NpgsqlConnection conn)
        {
            System.Console.WriteLine("Enter the first name of the student");
            string first_name = Console.ReadLine();

            System.Console.WriteLine("Enter the student's last name");
            string last_name = Console.ReadLine();

            System.Console.WriteLine("Enter the student's age");
            int age = 0;

            try
            {
                age = Convert.ToInt32(Console.ReadLine());
            }
            catch
            {
                System.Console.WriteLine("Enter a valid age");
            }

            System.Console.WriteLine("Now enter the student's birthday in the format YYYY-MM-DD");
            string input_1 = Console.ReadLine();
            DateTime birthday;
            if (DateTime.TryParse(input_1, out birthday))
            {
                using (var cmd = new NpgsqlCommand("INSERT INTO students (first_name, last_name, age, birthday) VALUES (@f, @l, @a, @b);", conn))
                {
                    cmd.Parameters.AddWithValue("@f", first_name);
                    cmd.Parameters.AddWithValue("@l", last_name);
                    cmd.Parameters.AddWithValue("@a", age);
                    cmd.Parameters.AddWithValue("@b", birthday);

                    cmd.ExecuteNonQuery();
                    System.Console.WriteLine("Student added successfully");
                    System.Console.WriteLine($"The student's details are {first_name},{last_name}, aged {age} years old and born on {birthday} ");
                }
            }
        }

        static void ShowStudents(NpgsqlConnection conn)
        {
            using (var readCmd = new NpgsqlCommand("SELECT * FROM students", conn))
            using (var reader = readCmd.ExecuteReader())
            {
                while (reader.Read())
                {
                    System.Console.WriteLine($"The students are: {reader["first_name"]} {reader["last_name"]} {reader["age"]} {reader["birthday"]} ");
                }
            }
        }

        static void UpdateStudent(NpgsqlConnection conn)
        {

            System.Console.WriteLine("What would you like to update?");
            System.Console.WriteLine("1. First_Name\n2. Last_Name\n3.Age\n4.Birthday");
            string input_2 = Console.ReadLine();

            if (input_2 == "1")
            {
                using (var readCmd = new NpgsqlCommand("SELECT * FROM students", conn))
                using (var reader = readCmd.ExecuteReader())
                {
                    while (reader.Read())
                    {
                        System.Console.WriteLine($"The students in your class are: {reader["id"]} {reader["first_name"]} {reader["last_name"]}");
                    }
                    System.Console.WriteLine("Enter the student id of the student you want to modify");
                    int idToModify = Convert.ToInt32(Console.ReadLine());
                    System.Console.WriteLine("Enter a new first name for student with ");
                    string newFirstName = Console.ReadLine();

                    using (var cmd = new NpgsqlCommand("UPDATE students SET first_name = @new WHERE id = @id", conn))
                    {
                        cmd.Parameters.AddWithValue("new", newFirstName);
                        cmd.Parameters.AddWithValue("id", idToModify);

                        int rowsAffected = cmd.ExecuteNonQuery();

                        if (rowsAffected == 0)
                        {
                            System.Console.WriteLine($"No student was found with id {idToModify}");
                        }
                        else
                        {
                            System.Console.WriteLine("Student updated successfully");
                        }

                    }
                }
            }
            else if (input_2 == "2")
            {
                using (var readCmd = new NpgsqlCommand("SELECT * FROM students", conn))
                using (var reader = readCmd.ExecuteReader())
                {
                    System.Console.WriteLine($"The students in your class are: {reader["id"]} {reader["first_name"]} {reader["last_name"]}");
                    System.Console.WriteLine("Enter the student id of the student you want to modify");
                    int idToModify = Convert.ToInt32(Console.ReadLine());
                    System.Console.WriteLine("Enter a new last name for student with id {idToModify} ");
                    string newLastName = Console.ReadLine();

                    using (var cmd = new NpgsqlCommand("UPDATE students SET last_Name = @new WHERE id = @id"))
                    {
                        cmd.Parameters.AddWithValue("new", newLastName);

                        int rowsAffected = cmd.ExecuteNonQuery();

                        if (rowsAffected == 0)
                        {
                            System.Console.WriteLine($"No student was found with id {idToModify}");
                        }
                        else
                        {
                            System.Console.WriteLine("Student updated successfully");
                        }
                    }
                }

            }
            else if (input_2 == "3")
            {
                using (var readCmd = new NpgsqlCommand("SELECT * FROM students", conn))
                using (var reader = readCmd.ExecuteReader())
                {
                    System.Console.WriteLine($"The students in your class are: {reader["id"]} {reader["first_name"]} {reader["last_name"]} {reader["age"]}");
                    System.Console.WriteLine("Enter the student id of the student you want to modify");
                    int idToModify = Convert.ToInt32(Console.ReadLine());
                    System.Console.WriteLine($"Enter a new age for student with id {idToModify} ");
                    int newAge = Convert.ToInt32(Console.ReadLine());

                    using (var cmd = new NpgsqlCommand("UPDATE students SET age = @new WHERE id = @id"))
                    {
                        cmd.Parameters.AddWithValue("new", newAge);

                        int rowsAffected = cmd.ExecuteNonQuery();

                        if (rowsAffected == 0)
                        {
                            System.Console.WriteLine($"No student was found with id {idToModify}");

                        }
                        else
                        {
                            System.Console.WriteLine("Student updated successfully");
                        }
                    }

                }
            }
            else if (input_2 == "4")
            {
                using (var readCmd = new NpgsqlCommand("SELECT * FROM students", conn))
                using (var reader = readCmd.ExecuteReader())

                    System.Console.WriteLine($"The students in your class are: {reader["id"]} {reader["first_name"]} {reader["last_name"]} {reader["birthday"]}");
                System.Console.WriteLine("Enter the student id of the student you want to modify");
                int idToModify = Convert.ToInt32(Console.ReadLine());
                System.Console.WriteLine($"Enter a new birthday for student with id {idToModify} in the format YYYY-MM-DD ");
                DateTime newBirthday = DateTime.Parse(Console.ReadLine());

                using (var cmd = new NpgsqlCommand("UPDATE students SET birthday = @new WHERE id = @id"))
                {
                    cmd.Parameters.AddWithValue("new", newBirthday);

                    var rowsAffected = cmd.ExecuteNonQuery();

                    if (rowsAffected == 0)
                    {
                        System.Console.WriteLine($"There is no student with id {idToModify}");
                    }
                    else
                    {
                        System.Console.WriteLine("Student updated successfully");
                    }
                }


            }
        }
    }
    
    static void DeleteStudent(NpgsqlConnection conn)
    {
        
    }
}