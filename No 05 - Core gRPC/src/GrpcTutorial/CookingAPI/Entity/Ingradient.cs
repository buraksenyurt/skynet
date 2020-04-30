namespace Cooking.Entity
{
    public class Ingradient
    {
        public int IngradientId { get; set; }
        public string Description { get; set; }
        public int RecipeId { get; set; }
        public Recipe Recipe { get; set; }
    }
}
