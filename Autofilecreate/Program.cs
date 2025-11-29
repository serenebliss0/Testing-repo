using System.IO;
using System.Threading.Tasks;
class Program
{
    static void Main(string[] args)
    {
        System.Console.WriteLine("Welcome to Autofilecreate!");
        System.Console.WriteLine("This program will help you create all the rust source files you need for your project.");
        System.Console.WriteLine("How many practice files do you want to create?");

        uint NumOfPracticeFiles = Convert.ToUInt32(System.Console.ReadLine());
        System.Console.WriteLine("Type in the folder directory where you want to create the practice files");
        string directoryPath = System.Console.ReadLine();

        if (string.IsNullOrWhiteSpace(directoryPath))
        {
            System.Console.WriteLine("Bruh i can't just make a file without it having a directory");
            System.Console.WriteLine("Try again");
            return;
        }
        Directory.SetCurrentDirectory(directoryPath);

        if (NumOfPracticeFiles == 0)
        {
            System.Console.WriteLine("Huh, I guess there's nothing for me to do. Goodbye!");
            return;
        }
        else
        {
            for (uint i = 1; i <= NumOfPracticeFiles; i++)
        {
            System.Diagnostics.Process.Start("cargo", $"new practice_{i}");

        }
        System.Console.WriteLine("All practice directories have been created successfully.");
        }

        Thread.Sleep(2000); //go to sleep little one for 2 seconds
        System.Console.WriteLine("Now, how many project files do you want to create?");
        uint NumOfProjectFiles = Convert.ToUInt32(System.Console.ReadLine());

        if (NumOfProjectFiles == 0)
        {
            System.Console.WriteLine("No project files to create. Exiting program.");
            return;
        }
        else
        {
            for (uint j = 1; j <= NumOfProjectFiles; j++)
            {
                System.Diagnostics.Process.Start("cargo", $"new project_{j} --bin");
            }
            System.Console.WriteLine("All project directories have been created successfully.");
        }
    }
}