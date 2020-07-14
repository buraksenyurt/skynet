using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;
using GamerMVC.Models;
using NorthwindLib;
using Microsoft.EntityFrameworkCore;

namespace GamerMVC.Controllers
{
    public class GameController : Controller
    {
        private readonly ILogger<GameController> _logger;
        private Northwind _db;
        public GameController(ILogger<GameController> logger, Northwind db)
        {
            _logger = logger;
            _db = db;
        }

        public async Task<IActionResult> Index()
        {
            // Geriye döndüreceğimiz Model nesnesini örnekliyoruz.
            // İçinde tüm oyunları, çıkış tarihlerine ait bir listeyi ve toplam oyun sayısını döndürüyoruz.
            // Aslında Index.cshtml için gerekli olan temel verilerimizi içeren bir model tasarladık diyebiliriz.
            GameIndexViewModel data = new GameIndexViewModel();
            data.Games = await _db.Games.ToListAsync();
            data.Years = await _db.Games.Select(g => g.Year).Distinct().OrderBy(g => g).ToListAsync<short?>();

            // ViewData üstünden de View katmanına veri taşımamız mümkündür.
            // Örneğin toplam oyun sayısını View'a taşıyabiliriz.
            ViewData["TotalGamesCount"] = await _db.Games.CountAsync();
            return View(data);
        }
    }
}
