using Alfabe.Core;
using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Mvc.RazorPages;
using Newtonsoft.Json;
using System.Net.Http;
using System.Text;
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
        //public void OnGet()
        //{

        //}

        public async Task<IActionResult> OnPostAsync()
        {
            if (!ModelState.IsValid)
            {
                return Page();
            }

            var url = "http://localhost:5000/nba/api/teams";
            var json = JsonConvert.SerializeObject(Team);
            var data = new StringContent(json, Encoding.UTF8, "application/json");
            await _client.PostAsync(url, data);

            return RedirectToPage("./Index");
        }
    }
}