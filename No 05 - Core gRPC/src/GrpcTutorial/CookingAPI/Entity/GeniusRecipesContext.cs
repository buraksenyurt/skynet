using Microsoft.EntityFrameworkCore;

namespace Cooking.Entity
{
    public class GeniusRecipesContext
        :DbContext
    {
        public GeniusRecipesContext(DbContextOptions options)
			: base(options)
        {
        }
        public DbSet<Recipe> Recipes { get; set; }
        public DbSet<Ingradient> Ingradients { get; set; }
    }
}
