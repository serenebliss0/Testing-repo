using System;
using System.Collections;
using System.Diagnostics;
using System.Dynamic;
using System.Security.Cryptography.X509Certificates;
using System.Text.RegularExpressions;
using System.Threading;

class Program1
{
   static void Main()
   {
      int a = 5;
      int b = a;

      Console.WriteLine(b);
      Console.WriteLine(a);

      int[] numbers = new int[] { 1, 2, 3, 4, 5 };
      int[] numbers2 = new int[] { 1, 2, 3, 4, 5 };

      Console.WriteLine(numbers2[0]);

      for (int i = 0; i < numbers.Length; i++)
      {
         Console.WriteLine(numbers[i]);
      }

      // Start some processes (optional) and then kill them
      var notepad = Process.Start("notepad.exe");
      var p2 = Process.Start("calc.exe");

      Console.WriteLine("Calculator Started Successfully");
      Console.WriteLine("Notepad Started Successfully");

      if (notepad != null) Console.WriteLine($"Notepad ID: {notepad.Id}");
      if (p2 != null) Console.WriteLine($"Calculator ID: {p2.Id}");

      Thread.Sleep(1000);
      try
      {
         p2?.Kill();
         notepad?.Kill();
         Console.WriteLine("Processes Killed Successfully");
      }
      catch (Exception ex)
      {
         Console.WriteLine($"Error killing processes: {ex.Message}");
      }

      Testing.SayHello();
      var t = new Testing();
      t.SayBye();

      var friend = new Friend();
      Console.WriteLine(friend.name);

      var t2obj = new Testing2();
      Console.WriteLine(t2obj.name);

      // Run the thread-safe counter demo
      TestingThreads.RunDemo();
   }
}

class Testing
{
   public static void SayHello()
   {
      Console.WriteLine("Hello from Static Class");
   }

   public void SayBye()
   {
      Console.WriteLine("Bye from Non Static Class");
   }
}

class Friend
{
   public string name = "Serenity";
   private int age = 16;
}

public class Testing2
{
   public string name = "Serenity";
   public int age = 16;
}


//Threads and processes look weird
class TestingThreads
{
   private static int counter = 0;

   public static void RunDemo()
   {
      Thread t1 = new Thread(Increment);
      Thread t2 = new Thread(Increment);

      t1.Start();
      t2.Start();

      t1.Join();
      t2.Join();

      Console.WriteLine(counter);
   }

   static void Increment()
   {
      for (int i = 0; i < 100000; i++)
      {
         Interlocked.Increment(ref counter);
      }
   }

   
}

class random
{
   public static int GenerateRandomNumber(int min, int max)
   {
      Random rand = new Random();
      return rand.Next(min, max);
      
   }

   
}

//checks which case is valid