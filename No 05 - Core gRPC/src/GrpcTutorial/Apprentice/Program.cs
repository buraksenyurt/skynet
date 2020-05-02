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

            #region Yemek Tarifi Ekleme

            //SendRecipeRequest request = new SendRecipeRequest
            //{
            //    Name = "Baked Shrimp Scampi",
            //    Source = "Ina Garten: Barefoot Contessa Back to Basics",
            //    Calories = 2565,
            //    Instructions = "Preheat the oven to 425 degrees F. Defrost shrimp by putting in cold water,drain.Place the shrimp in serving dish(9x13 or 2 quart casserole) and toss gently with the olive oil,wine,1 teaspoons salt,and 1 teaspoon pepper.Allow to sit at room temperature while you make the butter and garlic mixture. In a small bowl, mash the softened butter with the garlic, shallots, parsley, rosemary, red pepper flakes, lemon zest, lemon juice, egg yolk, panko, 1/2 teaspoon salt, and 1/4 teaspoon of pepper until combined. Spread the butter mixture evenly over the shrimp. Bake for 10 to 12 minutes until hot and bubbly.If you like the top browned, place under a broiler for 1 - 3 minutes(keep an eye on it).Serve with lemon wedges and French bread. Note: if using fresh shrimp, arrange for presentation.Starting from the outer edge of a 14 - inch oval gratin dish, arrange the shrimp in a single layer cut side down with the tails curling up and towards the center of the dish.Pour the remaining marinade over the shrimp. "
            //};
            //request.Ingredients.AddRange(new List<string>
            //{
            //    "2/3 cup panko",
            //    "1/4 teaspoon red pepper flakes",
            //    "1/2 lemon, zested and juiced",
            //    "1 extra-large egg yolk",
            //    "1 teaspoon rosemary, minced",
            //    "3 tablespoon parsley, minced",
            //    "4 clove garlic, minced",
            //    "1/4 cup shallots, minced",
            //    "8 tablespoon unsalted butter, at room temperature",
            //    "2 tablespoon dry white wine",
            //    "Freshly ground black pepper",
            //    "Kosher salt",
            //    "3 tablespoon olive oil",
            //    "2 pound frozen shrimp"
            //});
            //var inserted = await client.SendRecipeAsync(request);
            //Console.WriteLine($"{inserted.Id.ToString()} numaralı tarif eklenmiştir.");

            #endregion

            var recipe = await client.GetRecipeAsync(new GetRecipeRequest { RecipeId = 4 });
            Console.WriteLine($"{recipe.Name}\n{recipe.Calories} K\n{recipe.Instructions}");
            foreach (var ingradient in recipe.Ingredients)
            {
                Console.WriteLine($"\t{ingradient}");
            }

            Console.ReadLine();
        }
    }
}
