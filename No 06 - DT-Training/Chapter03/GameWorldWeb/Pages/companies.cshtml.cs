using Microsoft.AspNetCore.Mvc.RazorPages;
using Microsoft.AspNetCore.Mvc;
using System.Collections.Generic;
using System.Linq;
using NorthwindLib;

namespace GameWorldWeb.Pages
{
    public class CompaniesModel : PageModel
    {
        public IEnumerable<string> Companies { get; set; }
        [BindProperty]
        public Company Company { get; set; }
        private Northwind _db;
        // Veritabanı nesnesini constructor üstünden enjekte ediyoruz
        public CompaniesModel(Northwind db)
        {
            _db = db;
        }
        public void OnGet()
        {
            ViewData["Title"] = "Northwind Retro Game Kataloğu - Üretici Firmalar";
            // Companies = new[]{
            //     "ZeniMax Media Inc","Take-Two Interactive Software Inc","Nintendo","Electronic Arts","Sierra","Valve","Rockstar Games","Activision Blizzard","Ubisoft","Sega Games Co."
            // };
            Companies = _db.Companies.Select(c => c.Name);
        }

        public IActionResult OnPost()
        {
            if(ModelState.IsValid)
            {
                _db.Companies.Add(Company);
                _db.SaveChanges();
                return RedirectToPage("/companies");
            }
            return Page();
        }
    }

}