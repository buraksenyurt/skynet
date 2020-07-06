using System.Collections.Generic;
using Microsoft.EntityFrameworkCore;

namespace NorthwindLib
{
    public class Northwind
        : DbContext
    {
        public DbSet<Company> Companies { get; set; }
        public DbSet<Game> Games { get; set; }

        public Northwind(DbContextOptions<Northwind> options)
        : base(options)
        {

        }
        protected override void OnModelCreating(ModelBuilder modelBuilder)
        {
            base.OnModelCreating(modelBuilder);
            modelBuilder.Entity<Company>()
            .Property(c => c.Name)
            .IsRequired()
            .HasMaxLength(20);

            modelBuilder.Entity<Company>()
            .HasMany(c => c.Games)
            .WithOne(p => p.Company);

            modelBuilder.Entity<Game>()
            .Property(c => c.Title)
            .IsRequired()
            .HasMaxLength(100);

            modelBuilder.Entity<Game>()
            .HasOne(p => p.Company)
            .WithMany(c => c.Games);
        }
    }
}