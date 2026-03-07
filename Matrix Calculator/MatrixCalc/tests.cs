/*

// Matrix Calculator

using System.Collections;

System.Console.WriteLine("Welcome to the Matrix Calculator!");
System.Console.WriteLine("This program will help you perform various operations on matrices.");
System.Console.WriteLine("How many matrices are you working with?");

int matrixCount = Convert.ToInt32(Console.ReadLine());

for (int i = 1; i <= matrixCount; i++)
{
    System.Console.WriteLine($"Please enter the number of rows for matrix {i}:");
    int rows = Convert.ToInt32(Console.ReadLine());

    System.Console.WriteLine($"Please enter the number of columns for matrix {i}:");
    int columns = Convert.ToInt32(Console.ReadLine());

    if (rows <= 0 || columns <= 0)
    {
        System.Console.WriteLine("Invalid input. Please enter positive integers for rows and columns.");
        continue;
    }

    // Process the matrix based on its dimensions
    if (rows == 2 && columns == 2)
    {
        System.Console.WriteLine("This is a 2x2 matrix. You can perform operations like addition, subtraction, and multiplication.");
        ProcessMatrix(rows, columns);
    }
    else if (rows == 1 && columns == 1)
    {
        System.Console.WriteLine("This is a 1x1 matrix. You can perform operations like addition, subtraction, and multiplication.");
        ProcessMatrix(rows, columns);
    }
    else if (rows == 3 && columns == 3)
    {
        System.Console.WriteLine("This is a 3x3 matrix. You can perform operations like addition, subtraction, and multiplication.");
        ProcessMatrix(rows, columns);
    }
    else if (rows == 1 && columns == 2)
    {
        System.Console.WriteLine("This is a 1x2 matrix. You can perform operations like addition, subtraction, and multiplication.");
        ProcessMatrix(rows, columns);
    }
    else if (rows == 2 && columns == 1)
    {
        System.Console.WriteLine("This is a 2x1 matrix. You can perform operations like addition, subtraction, and multiplication.");
        ProcessMatrix(rows, columns);
    }
    else if (rows == 1 && columns == 3)
    {
        System.Console.WriteLine("This is a 1x3 matrix. You can perform operations like addition, subtraction, and multiplication.");
        ProcessMatrix(rows, columns);
    }
    else if (rows == 3 && columns == 1)
    {
        System.Console.WriteLine("This is a 3x1 matrix. You can perform operations like addition, subtraction, and multiplication.");
        ProcessMatrix(rows, columns);
    }
    else if (rows == 2 && columns == 3)
    {
        System.Console.WriteLine("This is a 2x3 matrix. You can perform operations like addition, subtraction, and multiplication.");
        ProcessMatrix(rows, columns);
    }
    else if (rows == 3 && columns == 2)
    {
        System.Console.WriteLine("This is a 3x2 matrix. You can perform operations like addition, subtraction, and multiplication.");
        ProcessMatrix(rows, columns);
    }
    else
    {
        System.Console.WriteLine("This is a matrix with more than 3 rows or columns. You can perform basic operations like addition and subtraction.");
        ProcessMatrix(rows, columns);
    }
}

// Method to handle matrix creation and operations
void ProcessMatrix(int rows, int columns)
{
    int[,] matrix = new int[rows, columns];

    // Fill the matrix
    for (int r = 0; r < rows; r++)
    {
        for (int c = 0; c < columns; c++)
        {
            System.Console.WriteLine($"Please enter the element at position ({r + 1}, {c + 1}):");
            matrix[r, c] = Convert.ToInt32(Console.ReadLine());
        }
    }

    // Ask for the operation
    System.Console.WriteLine("What operation would you like to perform on this matrix?");
    System.Console.WriteLine("1. Addition \n2. Subtraction \n3. Multiplication \n4. Transpose \n5. Determinant \n6. Inverse \n7. Exit");
    int operation = Convert.ToInt32(Console.ReadLine());

    switch (operation)
    {
        case 1:
            System.Console.WriteLine("You have chosen addition.");
            System.Console.WriteLine($"Performing addition on a {rows}x{columns} matrix...");
            // Add your addition logic here
            break;

        case 2:
            System.Console.WriteLine("You have chosen subtraction.");
            System.Console.WriteLine($"Performing subtraction on a {rows}x{columns} matrix...");
            // Add your subtraction logic here
            break;

        case 3:
            System.Console.WriteLine("You have chosen multiplication.");
            System.Console.WriteLine($"Performing multiplication on a {rows}x{columns} matrix...");
            // Add your multiplication logic here
            break;

        case 4:
            System.Console.WriteLine("You have chosen transpose.");
            System.Console.WriteLine($"Transposing a {rows}x{columns} matrix...");
            // Add your transpose logic here
            break;

        case 5:
            System.Console.WriteLine("You have chosen determinant.");
            if (rows == columns)
            {
                System.Console.WriteLine($"Calculating determinant of a {rows}x{columns} matrix...");
                // Add your determinant logic here
            }
            else
            {
                System.Console.WriteLine("Determinant can only be calculated for square matrices.");
            }
            break;

        case 6:
            System.Console.WriteLine("You have chosen inverse.");
            if (rows == columns)
            {
                System.Console.WriteLine($"Calculating inverse of a {rows}x{columns} matrix...");
                // Add your inverse logic here
            }
            else
            {
                System.Console.WriteLine("Inverse can only be calculated for square matrices.");
            }
            break;

        case 7:
            System.Console.WriteLine("Exiting the program. Goodbye!");
            return; // Exit the method
    }
}
*/