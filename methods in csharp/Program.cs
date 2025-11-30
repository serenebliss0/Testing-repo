
class Quadratics
{
    public void solve_quadratics()
    {
        while(true)
        {
            System.Console.WriteLine("Enter coefficient a");
            double a;
            if (Double.TryParse(Console.ReadLine(), out a))
                {
                    if (a == 0)
                    {
                        Console.WriteLine("a cannot be 0. Try again!");
                        continue;
                    }
                    break; // valid a
                }
                else
                {
                    Console.WriteLine("Invalid input! Enter a valid number.");
                }
            }
            System.Console.WriteLine("Enter coefficient b");
            double b  = Convert.ToDouble(Console.ReadLine());
            System.Console.WriteLine("Enter coefficient c");
            double c = Convert.ToDouble(Console.ReadLine());
        }
    }

    class Program
{
    static void Main()
    {
        Quadratics q = new Quadratics();
        q.solve_quadratics();
    }
}