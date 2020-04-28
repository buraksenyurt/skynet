using Alfabe.Core;
using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Mvc.RazorPages;
using System.Net.Http;
using System.Threading.Tasks;

namespace Alfabe.Client.Pages
{
    public class CreateModel : PageModel
    {
        [BindProperty]
        public Team Team { get; set; }

        private readonly HttpClient _client;

        public CreateModel()
        {
            _client = new HttpClient();
        }

        public async Task<IActionResult> OnPostAsync()
        {
            if (!ModelState.IsValid)
            {
                return Page();
            }

            Proxy client = new Proxy();
            await client.CreateAsync(Team);

            return RedirectToPage("./Index");
        }
    }
}