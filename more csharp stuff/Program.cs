using System;

class Person
{
    public string Name { get; set; }

    public Person(string name)
    {
        Name = name;
        Console.WriteLine($"{Name} was created!");
    }

    ~Person() // finalizer (called when object is collected)
    {
        Console.WriteLine($"{Name} was destroyed!");
    }
}

class Program
{
    static void Main()
    {
        Person p = new Person("Serenity");
        p = null; // we lose reference to the Person object

        // Force garbage collection (for demo only)
        GC.Collect();
        GC.WaitForPendingFinalizers();

        Console.WriteLine("Garbage collection complete!");
    }
}
