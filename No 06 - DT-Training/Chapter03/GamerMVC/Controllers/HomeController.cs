using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;
using GamerMVC.Models;
using NorthwindLib; //Eklendi

namespace GamerMVC.Controllers
{
    /*
        HomeIndexViewModel isimli modeli kullanacak olan Controller sınıfımız
        Northwind isimli DbContext tipini kullanabilmesi için,
        Constructor üstünden enjekte ediyoruz. 
        İlgili Entity servisinin kayıt işlemi bildiğiniz üzere Startup.cs içerisinde yapılmıştı.
    */
    public class HomeController : Controller
    {
        private readonly ILogger<HomeController> _logger;
        private Northwind _db;
        public HomeController(ILogger<HomeController> logger, Northwind db)
        {
            _logger = logger;
            _db = db;
        }

        /*
            Index isimli action gerçekleştiğinde devreye giren metot.
            rModel isimli HomeIndexViewModel nesnesi tanımlayıp içerisine
            View tarafına göndermek istediğimiz model nesnelerini koyduk.

            Örneğimizde tüm firma ve oyun bilgileri döndürülüyor.

            HomeController'ın Index Action'a kadar inen View sayfası,
            View altındaki Home klasöründe yer alan Index.cshtml dosyası.

            Dolayısıyla buradaki model nesnelerini Index.cshtml'de ele alabiliriz.
        */
        public IActionResult Index()
        {
            var rModel = new HomeIndexViewModel()
            {
                Companies = _db.Companies.ToList(),
                Games = _db.Games.ToList()
            };

            return View(rModel);
            //return View();
        }

        /*
            Home.cshtml'de firma ismine tıklanınca çalışan action metodumuz
        */
        public IActionResult CompanyGamesDetail(int? id)
        {
            /*
            asp-route-id ile gönderilen companyID değişkeni
            varsayılan route tanımlaması gereği otomatik olarak id ismiyle içeri alınır
            */
            if (id.HasValue) //Eğer bir değere sahipse
            {
                // LINQ sorgusu ile bu firmanın oyunlarının çekelim
                var games = (from g in _db.Games
                             where g.CompanyID == id.Value
                             select g).ToList();

                // Hiçbir sonuç yoksa HTTP 404 NotFound dönebiliriz
                if (games.Count() == 0)
                {
                    return NotFound("Bu numaraya bağlı oyunları bulamadım");
                }
                // Eğer sonuç varsa oyun listesini View'a verebiliriz
                return View(games);
            }
            return NotFound("Bir ID girilmeli");
        }

        /*
            Yeni bir oyun firması eklerken devreye giren action metodu
        */
        [HttpPost] // Yeni bilgiler POST metodu ile gönderileceği için
        public IActionResult CreateCompany(CompanyGameModel data)
        {
            // Eğer model veri doğrulama kuralı ihlalleri içeriyorsa
            if (!ModelState.IsValid)
            {
                // Hatalı olduğunu ve hata mesajlarını doldurup View'a döndürüyoruz.
                // View'da bu hatalar ekrana bastırılıyor.
                data.HasErrors = true;
                data.ValidationErrors = ModelState.Values.SelectMany(s => s.Errors).Select(e => e.ErrorMessage);
                return View(data);
            }

            Company company = new Company
            {
                Name = data.Name,
                Description = data.Description
            };
            _db.Companies.Add(company);
            _db.SaveChanges();

            Game game = new Game
            {
                Title = data.GameTitle,
                Year = data.GameYear,
                Popuplarity = data.GamePopularity,
                Discontinued = data.GameIsContinued,
                CompanyID = company.CompanyID
            };
            _db.Games.Add(game);
            _db.SaveChanges();

            return View(data);
        }

        /*
            CreateCompany.cshtml view'unu döndüren action metodumuz
        */
        public IActionResult CreateCompany()
        {
            return View();
        }
        public IActionResult Privacy()
        {
            return View();
        }

        [ResponseCache(Duration = 0, Location = ResponseCacheLocation.None, NoStore = true)]
        public IActionResult Error()
        {
            return View(new ErrorViewModel { RequestId = Activity.Current?.Id ?? HttpContext.TraceIdentifier });
        }
    }
}
