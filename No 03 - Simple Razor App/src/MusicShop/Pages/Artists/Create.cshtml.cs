using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Mvc.RazorPages;
using Microsoft.Extensions.Logging;
using MusicShop.Data;
using MusicShop.Models;

namespace MusicShop.Pages.Artists
{
    public class CreateModel : PageModel
    {
        private readonly ILogger<CreateModel> _logger;
        private readonly MusicShopContext _context;
        [BindProperty]
        public Artist Artist { get; set; }

        public CreateModel(ILogger<CreateModel> logger,MusicShopContext context)
        {
            _logger = logger;
            _context=context;
        }

         public async Task<IActionResult> OnPostAsync()
        {
            if (!ModelState.IsValid)
            {
                return Page();
            }

            _context.Artist.Add(Artist);
            await _context.SaveChangesAsync();

            return RedirectToPage("./Index");
        }
    }
}
