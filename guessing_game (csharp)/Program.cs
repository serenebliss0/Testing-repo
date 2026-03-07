using System.Diagnostics.CodeAnalysis;
System.Console.WriteLine(@"Pick a number between 1 and 100\nIf you guess correctly, you get a special prize!");
int guess = Convert.ToInt32(Console.ReadLine());
Random rand = new Random();
int random_number = rand.Next(1, 100); // Generates a random number between 1 and 100

if (guess == random_number)
{
    System.Console.WriteLine("Yay, you got the correct number");
}
else if (guess != random_number)
{
    System.Console.WriteLine("Lol you lagged. Try again bro");
    return;
}
else
{
    System.Console.WriteLine("Lol you entered rubbish");
    return;
}