using Cooking.Entity;
using Grpc.Core;
using System.Linq;
using Microsoft.Extensions.Logging;
using System.Collections.Generic;
using System.Threading.Tasks;
using Google.Protobuf.Collections;

namespace CookingAPI
{
    public class CookService : Cook.CookBase
    {
        private readonly ILogger<CookService> _logger;
        private readonly GeniusRecipesContext _context;
        public CookService(ILogger<CookService> logger, GeniusRecipesContext context)
        {
            _logger = logger;
            _context = context;
        }

        public override Task<SendRecipeResponse> SendRecipe(SendRecipeRequest request, ServerCallContext context)
        {
            SendRecipeResponse response = new SendRecipeResponse();
            Recipe recipe = new Recipe
            {
                Name = request.Name,
                Calories = request.Calories,
                Source = request.Source,
                Instructions = request.Instructions
            };
            recipe.Ingradients = new List<Ingradient>();
            foreach (var ing in request.Ingredients)
            {
                recipe.Ingradients.Add(new Ingradient { Description = ing, RecipeId = recipe.RecipeId });
            }
            _context.Recipes.Add(recipe);            
            _context.SaveChanges();
            response.Id = recipe.RecipeId;

            return Task.FromResult(response);
        }

        public override Task<GetRecipeResponse> GetRecipe(GetRecipeRequest request, ServerCallContext context)
        {
            var recipe=_context.Recipes.Where(r => r.RecipeId == request.RecipeId).SingleOrDefault();

            GetRecipeResponse response = new GetRecipeResponse
            {
                Id = recipe.RecipeId,
                Name = recipe.Name,
                Source = recipe.Source,
                Calories = recipe.Calories,
                Instructions = recipe.Instructions
            };

            return Task.FromResult(response);
        }
    }
}
