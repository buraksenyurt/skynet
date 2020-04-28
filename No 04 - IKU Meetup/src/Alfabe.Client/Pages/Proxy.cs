using Alfabe.Core;
using Newtonsoft.Json;
using System;
using System.Collections.Generic;
using System.Net.Http;
using System.Text;
using System.Threading.Tasks;

namespace Alfabe.Client.Pages
{
    internal class Proxy
    {
        private readonly HttpClient _client = new HttpClient();

        public Proxy()
        {
        }

        public async Task<Team> FindAsync(string id)
        {
            var url = $"http://localhost:5000/nba/api/teams/{id}";
            HttpResponseMessage response = await _client.GetAsync(url);
            response.EnsureSuccessStatusCode();
            var resp = await response.Content.ReadAsStringAsync();
            Team t = JsonConvert.DeserializeObject<Team>(resp);
            return t;
        }

        public async Task<IList<Team>> GetAllAsync()
        {
            var url = "http://localhost:5000/nba/api/teams";
            HttpResponseMessage response = await _client.GetAsync(url);
            response.EnsureSuccessStatusCode();
            var resp = await response.Content.ReadAsStringAsync();
            List<Team> teams = JsonConvert.DeserializeObject<List<Team>>(resp);
            return teams;
        }

        public async Task UpdateAsync(string id, Team team)
        {
            var url = $"http://localhost:5000/nba/api/teams/{id}";
            var json = JsonConvert.SerializeObject(team);
            var data = new StringContent(json, Encoding.UTF8, "application/json");
            await _client.PutAsync(url, data);
        }

        public async Task CreateAsync(Team team)
        {
            var url = "http://localhost:5000/nba/api/teams";
            var json = JsonConvert.SerializeObject(team);
            var data = new StringContent(json, Encoding.UTF8, "application/json");
            await _client.PostAsync(url, data);
        }

        public async Task<HttpResponseMessage> DeleteAsync(string id)
        {
            var url = $"http://localhost:5000/nba/api/teams/{id}";
            HttpResponseMessage response = await _client.DeleteAsync(url);
            return response;
        }
    }
}