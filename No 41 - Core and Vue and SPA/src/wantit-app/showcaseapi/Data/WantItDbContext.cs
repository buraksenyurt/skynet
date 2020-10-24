using Microsoft.EntityFrameworkCore;
using showcaseapi.Model;

namespace showcaseapi.Data
{
    public class WantItDbContext : DbContext
    {
        public WantItDbContext(DbContextOptions<WantItDbContext> options) : base(options)
        { }

        public DbSet<Clap> Claps { get; set; }
    }
}