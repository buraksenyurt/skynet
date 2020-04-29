using CookingAPI;
using Grpc.Net.Client;
using System;
using System.Threading.Tasks;

namespace Apprentice
{
    class Program
    {
        static async Task Main(string[] args)
        {
            using var channel = GrpcChannel.ForAddress("https://localhost:5555");
            var client = new Cook.CookClient(channel);
            var recipe = await client.GetRecipeAsync(
                              new GetRecipeRequest { Code = "1" });

            Console.WriteLine(recipe.ToString());
            Console.ReadLine();
        }
    }
}
