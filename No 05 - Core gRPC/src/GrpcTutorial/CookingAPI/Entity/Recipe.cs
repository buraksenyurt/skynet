using System.Collections.Generic;

namespace Cooking.Entity
{
    public class Recipe
    {
        public int RecipeId { get; set; }
        public string Name { get; set; }
        public string Source { get; set; }
        public int Calories { get; set; }
        public string Instructions { get; set; }
        public List<Ingradient> Ingradients { get; set; }
    }
}
