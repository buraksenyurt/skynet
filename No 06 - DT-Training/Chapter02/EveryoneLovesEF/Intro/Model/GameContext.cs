using Microsoft.EntityFrameworkCore;

namespace Intro.Model
{
    public class GameContext
        : DbContext
    {
        public DbSet<Player> Players { get; set; }

        protected override void OnConfiguring(DbContextOptionsBuilder options) => options.UseSqlite("Data Source=Game.db");
    }
}