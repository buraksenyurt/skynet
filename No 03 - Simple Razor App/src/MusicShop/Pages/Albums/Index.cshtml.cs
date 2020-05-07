using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.EntityFrameworkCore;
using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Mvc.RazorPages;
using Microsoft.Extensions.Logging;
using MusicShop.Data;
using MusicShop.Models;

namespace MusicShop.Pages.Albums
{
    public class IndexModel : PageModel
    {
        private readonly ILogger<IndexModel> _logger;
        private readonly MusicShopContext _context;
        public IList<Album> Albums{get;set;}

        public IndexModel(ILogger<IndexModel> logger,MusicShopContext context)
        {
            _logger = logger;
            _context=context;
        }

        public async Task OnGetAsync()
        {
            Albums=await _context.Album.Include(a=>a.Artist).ToListAsync();
        }

        public async Task<IActionResult> OnPostDeleteAsync(int id)
        {
            var album=await _context.Album.FirstOrDefaultAsync(a=>a.ID==id);
            if(album!=null)
            {
                _context.Album.Remove(album);
                await _context.SaveChangesAsync();
            }
            return RedirectToPage("./index");
        }
    }
}
