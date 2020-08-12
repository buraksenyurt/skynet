using System.Collections.Generic;
using Microsoft.EntityFrameworkCore;
using SmartWind.Models;

namespace SmartWind.Data
{
    public class Northwind
        : DbContext
    {
        public DbSet<Category> Categories { get; set; }
        public DbSet<Order> Orders { get; set; }
        public DbSet<Customer> Customers { get; set; }
        public DbSet<OrderDetail> OrderDetails { get; set; }
        public DbSet<Product> Products { get; set; }
        public Northwind(DbContextOptions<Northwind> options)
        : base(options)
        {
        }

        protected override void OnModelCreating(ModelBuilder modelBuilder)
        {
            base.OnModelCreating(modelBuilder);

            modelBuilder.Entity<Category>()
              .HasMany(c => c.Products)
              .WithOne(p => p.Category);

            modelBuilder.Entity<Customer>()
              .HasMany(c => c.Orders)
              .WithOne(o => o.Customer);

            modelBuilder.Entity<Product>()
              .HasOne(p => p.Category)
              .WithMany(c => c.Products);

            modelBuilder.Entity<OrderDetail>()
              .ToTable("Order Details");

            modelBuilder.Entity<OrderDetail>()
              .HasKey(od => new { od.OrderID, od.ProductID });
        }
    }
}