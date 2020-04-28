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
        private readonly Proxy _client;

        public IList<Team> Teams { get; private set; }

        public IndexModel(ILogger<IndexModel> logger)
        {
            _logger = logger;
            _client = new Proxy();
        }

        public async Task OnGetAsync()
        {
            Teams = await _client.GetAllAsync();            
        }

        public async Task<IActionResult> OnPostDeleteAsync(string id)
        {
             await _client.DeleteAsync(id);
            return RedirectToPage();
        }
    }
}
