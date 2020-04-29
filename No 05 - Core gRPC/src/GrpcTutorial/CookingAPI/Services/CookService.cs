using Grpc.Core;
using Microsoft.Extensions.Logging;
using System.Collections.Generic;
using System.Threading.Tasks;

namespace CookingAPI
{
    public class CookService : Cook.CookBase
    {
        private readonly ILogger<CookService> _logger;
        public CookService(ILogger<CookService> logger)
        {
            _logger = logger;
        }

        public override Task<GetRecipeReply> GetRecipe(GetRecipeRequest request, ServerCallContext context)
        {
            GetRecipeReply recipe = new GetRecipeReply
            {
                Id = 1,
                Name = "Tofu and Sweet Potato Jambalaya",
                Source = "Betty Crocker's Cookbook",
                Calories = 1590,
                Instructions = "1. Drain tofu; cut into 3/4-inch cubes. Carefully press cubes between paper towels to remove as much water as possible. 2. Heat oil in 12-inch skillet over medium heat. Cook tofu in oil 6 to 8 minutes, turning frequently, until light golden brown. Remove tofu from skillet; set aside. 3. Add sweet potato and garlic to skillet. Cook 2 to 3 minutes, stirring occasionally, just until sweet potato begins to brown. Stir in broth, rice, Worcestershire sauce and red pepper. Heat to boiling; reduce heat. Cover and simmer 10 minutes. 4. Stir in beans. Cover and cook 8 to 10 minutes, stirring occasionally, until rice is tender and liquid is absorbed. Stir in tofu and onions. Cook 1 to 2 minutes or until heated through.",
            };
            recipe.Ingredients.AddRange(new List<string>
            {
                "14 oz firm tofu",
                "1 tablespoon olive oil",
                "1 large sweet potato, peeled and cut into 1/2-inch cubes",
                "2 clove garlic, finely chopped",
                "2 cup vegetable broth",
                "3/4 cup uncooked regular long-grain white rice",
                "2 tablespoon Worcestershire sauce",
                "1/4 teaspoon ground red pepper",
                "1 can black beans (15 oz), rinsed and drained",
                "12 medium green onions, sliced"
            });
            return Task.FromResult(recipe);
        }
    }
}
