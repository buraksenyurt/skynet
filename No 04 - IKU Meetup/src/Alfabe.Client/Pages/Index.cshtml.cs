using Alfabe.Core;
using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Mvc.RazorPages;
using Microsoft.Extensions.Logging;
using Newtonsoft.Json;
using System.Collections.Generic;
using System.Net.Http;
using System.Threading.Tasks;

namespace Alfabe.Client.Pages
{
    public class IndexModel : PageModel
    {
        private readonly ILogger<IndexModel> _logger;
        private readonly HttpClient _client;
        public IList<Team> Teams { get; private set; }

        public IndexModel(ILogger<IndexModel> logger)
        {
            _logger = logger;
            _client = new HttpClient();
        }

        public async Task OnGetAsync()
        {
            var url = "http://localhost:5000/nba/api/teams";
            HttpResponseMessage response = await _client.GetAsync(url);
            response.EnsureSuccessStatusCode();
            var resp = await response.Content.ReadAsStringAsync();
            Teams = JsonConvert.DeserializeObject<List<Team>>(resp);
        }

        public async Task<IActionResult> OnPostDeleteAsync(string id)
        {
            var url = $"http://localhost:5000/nba/api/teams/{id}";
            HttpResponseMessage response = await _client.DeleteAsync(url);
            return RedirectToPage();
        }
    }
}
