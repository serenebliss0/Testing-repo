using System;
using System.Net.Http;
using System.Threading.Tasks;
using System.Text.Json; 
using System.Collections.Generic;

class Program
{
    static async Task Main()
    {
        string apiKey = "32b1358dca284e699947f4d84369c613";

        Console.WriteLine("Welcome user!");
        Console.WriteLine("What would you like to search for today?");
        Console.WriteLine("Type in your food item and press Enter:");

        string userChoice = Console.ReadLine();

        string url = $"https://api.spoonacular.com/recipes/complexSearch?query={userChoice}&apiKey={apiKey}";

        using (HttpClient client = new HttpClient())
        {
            string response = await client.GetStringAsync(url);

            // Deserialize JSON into your C# object
            SearchResult data = JsonSerializer.Deserialize<SearchResult>(response);

            Console.WriteLine($"\nHere are some recipes for {userChoice}:\n");

        int recipeId;
        
            if (data.results != null)
            {
                foreach (var recipe in data.results)
                {
                    Console.WriteLine($" {recipe.id}- {recipe.title}");
                }

            }
            else
            {
                Console.WriteLine("No recipes were found for your search.");
            }

            

        }
    }

    // Classes go here, OUTSIDE of Main()
    public class Recipe
    {
        public int id { get; set; }
        public string title { get; set; }
        public string image { get; set; }
    }

    public class SearchResult
    {
        public List<Recipe> results { get; set; }
    }
}
