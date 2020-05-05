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

namespace MusicShop.Pages.Artists
{
    public class DeleteModel : PageModel
    {
        private readonly ILogger<DeleteModel> _logger;
        private readonly MusicShopContext _context;
        [BindProperty]
        public Artist Artist{get;set;}

        public DeleteModel(ILogger<DeleteModel> logger,MusicShopContext context)
        {
            _logger = logger;
            _context=context;
        }

        public async Task<IActionResult> OnGetAsync(int id)
        {
            Artist=await _context.Artist.FirstOrDefaultAsync(a=>a.ID==id);
            if(Artist==null)
                return NotFound();
            
            return Page();
        }

        public async Task<IActionResult> OnPostAsync(int id)
        {
            Artist=await _context.Artist.FirstOrDefaultAsync(a=>a.ID==id);
            if(Artist!=null)
            {
                _context.Artist.Remove(Artist);
                await _context.SaveChangesAsync();
            }
            return RedirectToPage("./index");
        }
    }
}
