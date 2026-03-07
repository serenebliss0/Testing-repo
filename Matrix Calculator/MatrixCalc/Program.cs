using System;
using System.Collections.Generic;

class Program
{
    static void Main(string[] args)
    {
        System.Console.WriteLine("Welcome to the Matrix Calculator!");
        System.Console.WriteLine("This program will help you perform various operations on matrices.");
        System.Console.WriteLine("How many matrices are you working with?");

        int matrixCount = Convert.ToInt32(Console.ReadLine());
        List<int[,]> matrices = new List<int[,]>();

        // Input matrices
        for (int i = 1; i <= matrixCount; i++)
        {
            System.Console.WriteLine($"Please enter the number of rows for matrix {i}:");
            int rows = Convert.ToInt32(Console.ReadLine());

            System.Console.WriteLine($"Please enter the number of columns for matrix {i}:");
            int columns = Convert.ToInt32(Console.ReadLine());

            if (rows <= 0 || columns <= 0)
            {
                System.Console.WriteLine("Invalid input. Please enter positive integers for rows and columns.");
                i--; // Retry the current matrix
                continue;
            }

            int[,] matrix = InputMatrix(rows, columns);
            matrices.Add(matrix);
        }

        // Perform operations on matrices
        while (true)
        {
            System.Console.WriteLine("What operation would you like to perform?");
            System.Console.WriteLine("1. Addition \n2. Subtraction \n3. Multiplication \n4. Transpose \n5. Determinant \n6. Inverse \n7. Exit");
            int operation = Convert.ToInt32(Console.ReadLine());

            if (operation == 7)
            {
                System.Console.WriteLine("Exiting the program. Goodbye!");
                break;
            }

            if (matrices.Count < 1)
            {
                System.Console.WriteLine("No matrices available to perform operations.");
                continue;
            }

            switch (operation)
            {
                case 1:
                    PerformAddition(matrices);
                    break;
                case 2:
                    PerformSubtraction(matrices);
                    break;
                case 3:
                    PerformMultiplication(matrices);
                    break;
                case 4:
                    PerformTranspose(matrices);
                    break;
                case 5:
                    PerformDeterminant(matrices);
                    break;
                case 6:
                    PerformInverse(matrices);
                    break;
                default:
                    System.Console.WriteLine("Invalid operation. Please choose a valid option.");
                    break;
            }
        }
    }

    static int[,] InputMatrix(int rows, int columns)
    {
        int[,] matrix = new int[rows, columns];
        for (int r = 0; r < rows; r++)
        {
            for (int c = 0; c < columns; c++)
            {
                System.Console.WriteLine($"Please enter the element at position ({r + 1}, {c + 1}):");
                matrix[r, c] = Convert.ToInt32(Console.ReadLine());
            }
        }
        return matrix;
    }

    static void PerformAddition(List<int[,]> matrices)
    {
        if (matrices.Count < 2)
        {
            System.Console.WriteLine("Addition requires at least two matrices.");
            return;
        }

        int[,] matrix1 = matrices[0];
        int[,] matrix2 = matrices[1];

        if (matrix1.GetLength(0) != matrix2.GetLength(0) || matrix1.GetLength(1) != matrix2.GetLength(1))
        {
            System.Console.WriteLine("Matrices must have the same dimensions for addition.");
            return;
        }

        int rows = matrix1.GetLength(0);
        int columns = matrix1.GetLength(1);
        int[,] result = new int[rows, columns];

        for (int r = 0; r < rows; r++)
        {
            for (int c = 0; c < columns; c++)
            {
                result[r, c] = matrix1[r, c] + matrix2[r, c];
            }
        }

        System.Console.WriteLine("Result of addition:");
        PrintMatrix(result);
    }

    static void PerformSubtraction(List<int[,]> matrices)
    {
        if (matrices.Count < 2)
        {
            System.Console.WriteLine("Subtraction requires at least two matrices.");
            return;
        }

        int[,] matrix1 = matrices[0];
        int[,] matrix2 = matrices[1];

        if (matrix1.GetLength(0) != matrix2.GetLength(0) || matrix1.GetLength(1) != matrix2.GetLength(1))
        {
            System.Console.WriteLine("Matrices must have the same dimensions for subtraction.");
            return;
        }

        int rows = matrix1.GetLength(0);
        int columns = matrix1.GetLength(1);
        int[,] result = new int[rows, columns];

        for (int r = 0; r < rows; r++)
        {
            for (int c = 0; c < columns; c++)
            {
                result[r, c] = matrix1[r, c] - matrix2[r, c];
            }
        }

        System.Console.WriteLine("Result of subtraction:");
        PrintMatrix(result);
    }

    static void PerformMultiplication(List<int[,]> matrices)
    {
        if (matrices.Count < 2)
        {
            System.Console.WriteLine("Multiplication requires at least two matrices.");
            return;
        }

        int[,] matrix1 = matrices[0];
        int[,] matrix2 = matrices[1];

        if (matrix1.GetLength(1) != matrix2.GetLength(0))
        {
            System.Console.WriteLine("Number of columns in the first matrix must equal the number of rows in the second matrix.");
            return;
        }

        int rows = matrix1.GetLength(0);
        int columns = matrix2.GetLength(1);
        int common = matrix1.GetLength(1);
        int[,] result = new int[rows, columns];

        for (int r = 0; r < rows; r++)
        {
            for (int c = 0; c < columns; c++)
            {
                for (int k = 0; k < common; k++)
                {
                    result[r, c] += matrix1[r, k] * matrix2[k, c];
                }
            }
        }

        System.Console.WriteLine("Result of multiplication:");
        PrintMatrix(result);
    }

    static void PerformTranspose(List<int[,]> matrices)
    {
        int[,] matrix = matrices[0];
        int rows = matrix.GetLength(0);
        int columns = matrix.GetLength(1);
        int[,] result = new int[columns, rows];

        for (int r = 0; r < rows; r++)
        {
            for (int c = 0; c < columns; c++)
            {
                result[c, r] = matrix[r, c];
            }
        }

        System.Console.WriteLine("Transpose of the matrix:");
        PrintMatrix(result);
    }

    static void PerformDeterminant(List<int[,]> matrices)
    {
        int[,] matrix = matrices[0];
        if (matrix.GetLength(0) != matrix.GetLength(1))
        {
            System.Console.WriteLine("Determinant can only be calculated for square matrices.");
            return;
        }

        // Determinant logic (placeholder)
        System.Console.WriteLine("Determinant calculation is not implemented yet, but it will be in the future.");

        
    }

    static void PerformInverse(List<int[,]> matrices)
    {
        int[,] matrix = matrices[0];
        if (matrix.GetLength(0) != matrix.GetLength(1))
        {
            System.Console.WriteLine("Inverse can only be calculated for square matrices.");
            return;
        }

        // Inverse logic (placeholder)
        System.Console.WriteLine("Inverse calculation is not implemented yet but will be implemented soon.");
    }

    static void PrintMatrix(int[,] matrix)
    {
        int rows = matrix.GetLength(0);
        int columns = matrix.GetLength(1);

        for (int r = 0; r < rows; r++)
        {
            for (int c = 0; c < columns; c++)
            {
                System.Console.Write(matrix[r, c] + "\t");
            }
            System.Console.WriteLine();
        }
    }
}

//Completed by 29-MAR-2025 6:50 PM. 
//I will continue later.
//I still need to find a way to make the determinant and inverse functions work properly.
//I will also need to add error handling for invalid inputs and edge cases.
//Bye. Goodnight.