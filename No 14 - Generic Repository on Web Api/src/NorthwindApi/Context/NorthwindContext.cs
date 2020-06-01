using NorthwindApi.Models;
using Microsoft.EntityFrameworkCore;

namespace NorthwindApi.Context
{
    /*
    Tipik Entity Framework Context sınıfımız
    */
    public class NorthwindContext
        : DbContext
    {
        public DbSet<Category> Categories { get; set; }
        public DbSet<Product> Products { get; set; }

        public NorthwindContext(DbContextOptions<NorthwindContext> options)
        : base(options)
        {
        }
    }
}