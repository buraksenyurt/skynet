using Alfabe.Core;
using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Mvc.RazorPages;
using Newtonsoft.Json;
using System.Net.Http;
using System.Threading.Tasks;

namespace Alfabe.Client.Pages
{
    public class DetailModel : PageModel
    {
        public Team Team { get; set; }

        public async Task<IActionResult> OnGetAsync(string id)
        {
            HttpClient client = new HttpClient();
            var url = $"http://localhost:5000/nba/api/teams/{id}";
            HttpResponseMessage response = await client.GetAsync(url);
            response.EnsureSuccessStatusCode();
            var resp = await response.Content.ReadAsStringAsync();
            Team = JsonConvert.DeserializeObject<Team>(resp);

            if (Team == null)
            {
                return NotFound();
            }
            return Page();
        }
    }
}