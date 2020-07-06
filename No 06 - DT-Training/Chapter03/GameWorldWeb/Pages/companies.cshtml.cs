using Microsoft.AspNetCore.Mvc.RazorPages;
using System.Collections.Generic;
using System.Linq;
using NorthwindLib;

namespace GameWorldWeb.Pages
{
    public class CompaniesModel : PageModel
    {
        public IEnumerable<string> Companies { get; set; }
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
    }

}