using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Net;
using System.Net.Http;
using System.Net.Http.Headers;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;
using GameModel;
using Microsoft.Extensions.Caching.Distributed;
using Newtonsoft.Json;

namespace GameCore.Controllers
{
    [ApiController]
    [Route("core/api/[controller]")]
    public class CoreController : ControllerBase
    {
        private readonly ILogger<CoreController> _logger;
        private readonly IDistributedCache _distributedCache;
        private readonly HttpClient _client = new HttpClient();

        public CoreController(ILogger<CoreController> logger, IDistributedCache distributedCache)
        {
            _logger = logger;
            _distributedCache = distributedCache;
        }

        [HttpGet("{heroName}")]
        public async Task<List<Card>> GetAsync(string heroName)
        {
            List<Card> cardList;
            string jsonValue;
            // Önce Redis üzerinde gelen heroName key ile bir value var mı bakalım
            var cards = await _distributedCache.GetAsync(heroName);
            if (cards == null) // Redis'te yoksa cache'e al
            {
                _logger.LogWarning($"{DateTime.Now.ToLongTimeString()} -> Redis'te yoktu. O zaman önce oraya ekleyelim");

                // Önce diğer servisten kahramana göre kartları isteyelim
                var resp = await _client.GetAsync($"http://localhost:5005/api/card/{heroName}");
                // HTTP Get çağrısına dönecek JSON içeriği Card listesine çevirelim
                var stream = await resp.Content.ReadAsStringAsync();
                _logger.LogWarning($"Diğer servisten gelen içerik\n{stream}");

                cardList = JsonConvert.DeserializeObject<List<Card>>(stream);
                // Diğer servisten gelen json içeriği redis'te tutmak için UTF8 tipinden binary formata dönüştürelim.
                cards = Encoding.UTF8.GetBytes(stream);

                // Cache üzerinde saklama ayarları
                var options = new DistributedCacheEntryOptions()
                       .SetSlidingExpiration(TimeSpan.FromMinutes(5)) // Test amaçlı olarak 5 dakika erişilmediyse expire olacak
                       .SetAbsoluteExpiration(DateTime.Now.AddHours(1)); // Her durumda saatte bir expire edecek

                // key değeri kahramanın adı. İçerik cards değişkenin binary değeri(Kuvvetle muhtemel List<Card> içeriği). 
                // Seçenekler 5 dakika boyunca erişilmezse düşür ve her durumda saat başı yenile
                await _distributedCache.SetAsync(heroName, cards, options);
            }
            else // Redis'te varsa
            {
                _logger.LogWarning($"{DateTime.Now.ToLongTimeString()} -> Redis'te vermış. Oradan çekelim :)");

                // nesneyi UTF8 string'e çözümle
                jsonValue = Encoding.UTF8.GetString(cards);
                // şimdi güzelim Newtonsoft ile onu Card listesine dönüştür
                cardList = JsonConvert.DeserializeObject<List<Card>>(jsonValue);
            }

            return cardList;
        }
    }
}
