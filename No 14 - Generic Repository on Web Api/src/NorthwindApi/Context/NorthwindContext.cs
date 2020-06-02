using NorthwindApi.Models;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Logging.Console;

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

        // EF tarafından SQLite'a giden sorguları görmek için console'a loglama yapacağız
        public static readonly ILoggerFactory loggerFactory
            = LoggerFactory.Create(builder => { builder.AddConsole(); });
        public NorthwindContext(DbContextOptions<NorthwindContext> options)
        : base(options)
        {
        }

        // Context nesnesinin ilgili operasyonunu override edip
        // console loglayıcısını kullanmasını söylüyoruz
        protected override void OnConfiguring(DbContextOptionsBuilder optionsBuilder) => optionsBuilder.UseLoggerFactory(loggerFactory);
    }
}