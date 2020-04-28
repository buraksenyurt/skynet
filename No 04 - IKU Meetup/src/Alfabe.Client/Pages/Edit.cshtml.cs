using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.Http;
using System.Threading.Tasks;
using Alfabe.Core;
using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Mvc.RazorPages;

namespace Alfabe.Client.Pages
{
    public class EditModel : PageModel
    {
        private readonly Proxy _client;

        [BindProperty]
        public Team Team{ get; set; }

        public EditModel()
        {
            _client = new Proxy();
        }

        public async Task<IActionResult> OnGetAsync(string id)
        {
            Team = await _client.FindAsync(id);

            if (Team == null)
            {
                return RedirectToPage("./Index");
            }

            return Page();
        }

        public async Task<IActionResult> OnPostAsync(string id)
        {
            if (!ModelState.IsValid)
            {
                return Page();
            }

            await _client.UpdateAsync(id, Team);

            return RedirectToPage("./Index");
        }
    }
}