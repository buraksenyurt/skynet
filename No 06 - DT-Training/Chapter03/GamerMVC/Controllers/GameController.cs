using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;
using GamerMVC.Models;
using NorthwindLib;

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

        public IActionResult Index()
        {
            // Geriye döndüreceğimiz Model nesnesini örnekliyoruz.
            // İçinde tüm oyunları, çıkış tarihlerine ait bir listeyi ve toplam oyun sayısını döndürüyoruz.
            // Aslında Index.cshtml için gerekli olan temel verilerimizi içeren bir model tasarladık diyebiliriz.
            GameIndexViewModel data = new GameIndexViewModel();
            data.Games = _db.Games.ToList();
            data.Years = _db.Games.Select(g => g.Year).Distinct().OrderBy(g => g).ToList<short?>();

            // ViewData üstünden de View katmanına veri taşımamız mümkündür.
            // Örneğin toplam oyun sayısını View'a taşıyabiliriz.
            ViewData["TotalGamesCount"] = _db.Games.Count();
            return View(data);
        }
    }
}
