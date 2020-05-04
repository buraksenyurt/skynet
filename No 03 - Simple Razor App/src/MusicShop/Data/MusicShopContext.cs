using Microsoft.EntityFrameworkCore;
using System.Collections.Generic;
using MusicShop.Models;

namespace MusicShop.Data
{
    public class MusicShopContext
        : DbContext
    {
        public MusicShopContext(DbContextOptions<MusicShopContext> options)
            : base(options)
        {

        }

        public DbSet<Artist> Artist { get; set; }
        public DbSet<Album> Album { get; set; }
    }
}