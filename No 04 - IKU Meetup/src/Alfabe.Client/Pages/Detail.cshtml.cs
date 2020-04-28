using Alfabe.Core;
using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Mvc.RazorPages;
using System.Threading.Tasks;

namespace Alfabe.Client.Pages
{
    public class DetailModel : PageModel
    {
        public Team Team { get; set; }

        public async Task<IActionResult> OnGetAsync(string id)
        {
            Proxy client = new Proxy();
            Team = await client.FindAsync(id);

            if (Team == null)
            {
                return NotFound();
            }
            return Page();
        }
    }
}