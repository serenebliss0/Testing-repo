/*
class Program
{
    static void Main()
    {
        var connString = "Host=localhost;Username=postgres;Password=faith91#;Database=testing";

        using (var conn = new NpgsqlConnection(connString))
        {
            conn.Open();
            Console.WriteLine("Connected to Postgres!");

            using (var cmd = new NpgsqlCommand("CREATE TABLE IF NOT EXISTS students (id SERIAL PRIMARY KEY, name TEXT, age INT)", conn))
            {
                cmd.ExecuteNonQuery();
                Console.WriteLine("Table created!");
            }
            System.Console.WriteLine("Please enter your name");
            string name = Console.ReadLine();
            System.Console.WriteLine("Great! Now enter your age");
            int age = Convert.ToInt32(Console.ReadLine());

            using (var cmd = new NpgsqlCommand("INSERT INTO students (name,age) VALUES(@n,@a)", conn))
            {
                cmd.Parameters.AddWithValue("n", name);
                cmd.Parameters.AddWithValue("a", age);
                cmd.ExecuteNonQuery();
            }
            using (var readCmd = new NpgsqlCommand("SELECT * FROM students", conn))
            using (var reader = readCmd.ExecuteReader())
            {
                System.Console.WriteLine("\n All students");
                while (reader.Read())
                {
                    System.Console.WriteLine($"The students are : {reader["name"]} {reader["age"]}");
                }
            }

    
        }
    }
}
*/