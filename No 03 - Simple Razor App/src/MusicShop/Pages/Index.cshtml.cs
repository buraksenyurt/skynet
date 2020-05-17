using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Mvc.RazorPages;
using Microsoft.Extensions.Logging;
using Microsoft.EntityFrameworkCore;
using MusicShop.Data;
using MusicShop.Models;

namespace MusicShop.Pages
{
    public class IndexModel : PageModel
    {
        private readonly ILogger<IndexModel> _logger;
        private readonly MusicShopContext _context;
        public Album RandomAlbum { get; set; }

        public IndexModel(ILogger<IndexModel> logger, MusicShopContext context)
        {
            _logger = logger;
            _context = context;
        }

        public async Task OnGet()
        {
            Random randomizer = new Random();
            var albumIds = (from a in _context.Album
                            select a.ID).ToList();
            var index = randomizer.Next(0, albumIds.Count());
            var albumId = albumIds[index];
            var albums = await _context.Album.Include(a => a.Artist).ToListAsync();
            RandomAlbum = albums.First(a => a.ID == albumId);
        }
    }
}
