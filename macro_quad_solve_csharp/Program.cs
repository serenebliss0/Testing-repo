//Quadratic Equation solver(but in C# this time)
using System.Security.Cryptography;

while (true)
{
    System.Console.WriteLine("Please enter the coefficient for a");
    double a_input = Convert.ToDouble(Console.ReadLine());

    System.Console.WriteLine("Enter the coefficient of b");
    double b_input = Convert.ToDouble(Console.ReadLine());

    System.Console.WriteLine("Enter the coefficient of c");
    double c_input = Convert.ToDouble(Console.ReadLine());

    //discriminant time

    double discriminant = (b_input * b_input) - (4 * a_input * c_input);

    if (b_input > 0)
    {
        System.Console.WriteLine($"The expression is {a_input}x^2 +{b_input}x + {c_input}");
    }
    else
    {
        System.Console.WriteLine($"The expression is {a_input}x^2 {b_input} + {c_input}");
    }

    if (discriminant > 0.0)
    {
        double root_1 = (-b_input + discriminant) / (2 * a_input);
        double root_2 = (-b_input + discriminant) / (2 * a_input);
        System.Console.WriteLine($"Good news! Your two roots are: {root_1} and {root_2}");
    }
    else if (discriminant == 0)
    {
        double root_1 = (-b_input + discriminant) / (2 * a_input);
        System.Console.WriteLine($"The equation has only one root which is {root_1}");
    }
    else
    {
        System.Console.WriteLine("No real roots");
        System.Console.WriteLine("The imaginary roots of the equation are\n");

        double real_part = (-b_input / (2.0 * a_input));
        double imaginary_part = (Math.Sqrt(-discriminant) / (2.0 * a_input));

        System.Console.WriteLine($"{real_part:F2} + {imaginary_part:F2}i and {real_part:F2} - {imaginary_part:F2}i ");

        //allow the user to re-run the code
        System.Console.WriteLine("Do you want to find the roots of another equation");
        System.Console.WriteLine("Type y or n");
        string option_1 = Console.ReadLine();

        if (option_1 != "y")
        {
            System.Console.WriteLine("Goodbye");
            break;
        }
    }
}