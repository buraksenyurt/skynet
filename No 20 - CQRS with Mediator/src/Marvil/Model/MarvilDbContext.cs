using Microsoft.EntityFrameworkCore;

namespace Marvil.Model
{
    public class MarvilDbContext : DbContext
    {
        public MarvilDbContext(DbContextOptions options) : base(options)
        {
        }
        public DbSet<Hero> Heroes { get; set; }
    }
}