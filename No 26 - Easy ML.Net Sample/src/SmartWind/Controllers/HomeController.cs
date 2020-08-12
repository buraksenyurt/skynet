using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;
// ML kullanımı için gerekli kütüphaneler
using Microsoft.ML;
using Microsoft.ML.Data;
using Microsoft.Data;
using Microsoft.ML.Trainers;

using Microsoft.AspNetCore.Hosting;
using Microsoft.EntityFrameworkCore;
using System.IO;
using System.Text;

using SmartWind.Data;
using SmartWind.Models;

namespace SmartWind.Controllers
{
    public class HomeController : Controller
    {
        private readonly ILogger<HomeController> _logger;
        private readonly Northwind _db;
        private readonly IWebHostEnvironment _hostEnv;
        private string[] countries = { "Germany", "USA", "UK" };
        /*
            Constructor'dan EF Context ve WebHostEnvironment'i(Data folder'ını bulmak için) enjekte ettik
        */
        public HomeController(ILogger<HomeController> logger, Northwind db, IWebHostEnvironment hostEnv)
        {
            _logger = logger;
            _db = db;
            _hostEnv = hostEnv;
        }

        /*
            Seçilen verisetinin fiziki adresini döndürmeye yardımcı olan metot.
        */
        private string GetDataSetPath(string file)
        {
            return Path.Combine(_hostEnv.ContentRootPath, "wwwroot", "DataSets", file);
        }

        private HomeIndexViewModel CreateHomeIndexViewModel()
        {
            return new HomeIndexViewModel
            {
                Categories = _db.Categories.Include(c => c.Products),
                GermanyDatasetExists = System.IO.File.Exists(GetDataSetPath("germany-dataset.txt")),
                UKDatasetExists = System.IO.File.Exists(GetDataSetPath("uk-dataset.txt")),
                USADatasetExists = System.IO.File.Exists(GetDataSetPath("usa-dataset.txt"))
            };
        }

        public IActionResult Index()
        {
            /*
                Kategorileri, ürünleri ile birlikte döndüren 
                ve hatta DataSets klasörü içerisinde ülke bazında verisetleri
                olup olmadığı bilgilerini de içeren HomeIndexViewModel 
                nesnesini örnekleyip View tarafına gönderiyoruz.
            */
            var model = CreateHomeIndexViewModel();
            return View(model);
        }

        /*
            ML için veri setlerini örnekleyen ve sonrasında
            HomeIndexViewModel nesnesini oluşturup döndüren metot.
            Metot LING sorgusu yardımıyla 
            ProductID - RelatedProductID
            ikililerinden oluşan bir liste hazırlar. 
            Ülke bazında hazırlanan bu liste germany-dataset.txt,
            usa-dataset.txt, uk-dataset.txt adları ile
            wwwroot altındaki datasets klasörüne yazılır.

        */
        public IActionResult GenerateDataSets()
        {
            foreach (string country in countries) // Tanımlanan üç ülke için
            {
                // Bu ülkedeki siparişleri al
                var orders = _db.Orders
                            .Where(o => o.Customer.Country == country)
                            .Include(o => o.OrderDetails)
                            .AsEnumerable();

                // Ülke siparişlerindeki ürünler arası ilişkileri bul
                var productRelations = orders
                .SelectMany(
                    o =>
                    from item1 in o.OrderDetails
                    from item2 in o.OrderDetails
                    select new ProductRelation
                    {
                        ProductID = (uint)item1.ProductID,
                        RelatedProductID = (uint)item2.ProductID
                    }
                ).Where(p => p.ProductID != p.RelatedProductID)
                .GroupBy(p => new { p.ProductID, p.RelatedProductID })
                .Select(p => p.FirstOrDefault())
                .OrderBy(p => p.ProductID)
                .ThenBy(p => p.RelatedProductID);

                // Oluşturulan veriyi text dosyaya yaz
                StringBuilder builder = new StringBuilder();
                builder.AppendLine("ProductID\tRelatedProuductID");
                foreach (var p in productRelations)
                {
                    builder.AppendLine($"{p.ProductID}\t{p.RelatedProductID}");
                }
                System.IO.File.WriteAllText(GetDataSetPath($"{country}-dataset.txt"), builder.ToString());
            }

            // Modeli oluşturup View'a döndür
            // Yukarıdaki döngü çalışınca ülke bazlı veri setleri de hazır olacaktır
            var model = CreateHomeIndexViewModel();
            return View("Index", model);
        }

        /*
            Modeli eğitilmesi için kullanılan Action metodu.
            Matrix Factorization (Collaborative Filtering olarak da geçiyor) algoritması kullanılır.
        */
        public IActionResult TrainModels()
        {
            foreach (string country in countries)
            {
                var mlContext = new MLContext();

                // Algoritma için girdi verisini taşıyan IDataView örneği hazırlanır

                var dataView = mlContext.Data.LoadFromTextFile( // Dosyadan yükleyecek
                  path: GetDataSetPath($"{country}-dataset.txt"), // veriseti dosyasını belirtiyoruz
                  columns: new[] // column ve row bilgilerini tanımlıyoruz
                  {
                    new TextLoader.Column(
                    name:     "Label",
                    dataKind: DataKind.Double,
                    index:    0),

                    new TextLoader.Column(
                    name:     "ProductID",
                    dataKind: DataKind.UInt32,
                    source:   new [] { new TextLoader.Range(0) },
                    keyCount: new KeyCount(200)),

                    new TextLoader.Column(
                    name:     "RelatedProductID",
                    dataKind: DataKind.UInt32,
                    source:   new [] { new TextLoader.Range(1) },
                    keyCount: new KeyCount(200))
                    },
                    hasHeader: true,
                    separatorChar: '\t'); // Kolonları Tab ile ayırmıştık hatırlarsanız

                /*
                    Algoritmaya has ayarlar. Buraları anlamak için algoritmanın detaylarını öğrenmem lazım.
                    Alphe, Lambda ve C değerleri ne anlama geliyor. Neden bu değerler verilmiş araştıralım.
                */
                var options = new MatrixFactorizationTrainer.Options
                {
                    MatrixColumnIndexColumnName = "ProductID",
                    MatrixRowIndexColumnName = "RelatedProductID",
                    LabelColumnName = "Label",
                    LossFunction = MatrixFactorizationTrainer.LossFunctionType.SquareLossOneClass,
                    Alpha = 0.01,
                    Lambda = 0.025,
                    C = 0.00001
                };

                MatrixFactorizationTrainer coachCarter = mlContext.Recommendation()
                  .Trainers.MatrixFactorization(options);

                ITransformer kokoskov = coachCarter.Fit(dataView); // Model eğitilir

                /* 
                    Üretilen model zip uzantılı kaydedilir.
                    Bu zip'i alıp başka bir uygulamada da kullanabiliriz.
                    Tabii veri setinin değişmesi halinde modeli yeniden eğitmek gerekecektir.
                */

                mlContext.Model.Save(kokoskov,
                  inputSchema: dataView.Schema,
                  filePath: GetDataSetPath($"{country}-model.zip"));
            }

            // Modelin ne kadar sürede eğitildiğini bulmak için buraya bir Stopwatch kullanımı getirilebilir ;)
            var model = CreateHomeIndexViewModel();
            return View("Index", model);
        }

        /*
            Ürün kartı gösteren Action Metodu.
            Burayı yazarken yer yer beynim yandı.
        */
        public IActionResult Cart(int? id)
        {
            // O anki Cart bilgisini cookie'de saklıyor
            string cartCookie = Request.Cookies["basket_items"] ?? string.Empty;

            /*
                Sepete eklenen ürünler bu örnek özelinde bir cookie'de duruyorlar.
                Cart action metoduna gelen id değeri boş değilse 
            */
            if (id.HasValue)
            {
                if (string.IsNullOrWhiteSpace(cartCookie))
                {
                    cartCookie = id.ToString();
                }
                else // ve ürün sepeti çerezinin içerisinde veriler varsa
                {
                    string[] ids = cartCookie.Split('|'); // pipe karakterine göre içeriği split ediyoruz 

                    if (!ids.Contains(id.ToString())) // gelen id bu çerez içerisinde yoksa(yani ürün sepette değilse)
                    {
                        cartCookie = string.Join('|', cartCookie, id.ToString()); // çerezin sonuna ürün numarasını (ProductID) ekliyoruz
                    }
                }

                // Çeresin güncel halinide basket_items anahtar değeri ile Response.Cookies koleksiyonuna ekliyoruz
                Response.Cookies.Append("basket_items", cartCookie);
            }

            // Önerileri ve güncel sepet içeriğini tutan model nesnesini örnekliyoruz
            // İlerleyen aşamalarda Recommendations ile belirtilen öneriler kısmı da doldurulacak
            var model = new HomeCartViewModel
            {
                Cart = new Cart
                {
                    Items = Enumerable.Empty<CartItem>()
                },
                Recommendations = new List<EnrichedRecommendation>()
            };

            // Çerez içeriğini ele aldığımız kısım
            if (cartCookie.Length > 0)
            {
                /*
                    Çerez listesini pipe işaretine göre böldükten sonra
                    Her bir ID'yi ve bundan yararlanarak bulacağımız ürün adını
                    CartItem nesnelerini örneklemek için kullanıyoruz
                    dolayısıyla Cart modelindeki Items koleksiyonunu çerezdeki ürün bilgileri ile doldurmuş olduk
                */
                model.Cart.Items = cartCookie.Split('|').Select(item =>
                  new CartItem
                  {
                      ProductID = int.Parse(item),
                      ProductName = _db.Products.Find(int.Parse(item)).ProductName
                  });
            }

            /*
                Şimdi eğitilmiş modelimizi devreye almaktayız.
                uk-model.zip'i kullanıyoruz. TrainModels bizim için gerekli model eğitimlerini
                tamamlayıp ülkelere göre ayrı zip dosyalarının oluşturulmasını sağlamıştı.
            */
            if (System.IO.File.Exists(GetDataSetPath("uk-model.zip"))) // UK Model eğitilmişse
            {
                var mlContext = new MLContext(); // MLContext nesnesi

                ITransformer modelUK;

                // uk-model.zip dosyasını kullanarak tahminleme motoru için gerekli model nesnesini yüklüyoruz
                using (var stream = new FileStream(
                  path: GetDataSetPath("uk-model.zip"),
                  mode: FileMode.Open,
                  access: FileAccess.Read,
                  share: FileShare.Read))
                {
                    modelUK = mlContext.Model.Load(stream, out DataViewSchema schema);
                }

                // Burası önemli! Tahminleme motorunu aktifleştiriyoruz
                var predictionEngine = mlContext.Model.CreatePredictionEngine<ProductRelation, Recommendation>(modelUK);

                // Şimdi var olan ürün listesini ele alalım
                var products = _db.Products.ToArray();

                /*
                    Sepete eklenen her ürün için tahmin motorunu kullanarak öneriler alınacak.
                    Bu öneriler Modelimizdeki Recommendations isimli liste üzerinde değerlendiriliyor.
                    Ekleme sırasında yapılan skorlamaya göre en olası 3 ürün Recommendations listesinde bırakılıyor.
                */
                foreach (var item in model.Cart.Items) // Çerezlerden yüklenen ürün listesindeki herbir öğeyi al
                {
                    /*
                        Ürünlerdeki ProductID değerini RelatedProductID olarak alıp çerezden gelen listedeki ProductID ile
                        ilişkilendirip tahminleme motorundan bir tahminleme yapmasını istiyoruz.
                        Bu ilişki skorlara göre tersten sıralanıyor ve ilk üçü alınıyor. Yani en olası üçlü.
                    */
                    var topThree = products
                      .Select(product =>
                        predictionEngine.Predict(
                          new ProductRelation
                          {
                              ProductID = (uint)item.ProductID,
                              RelatedProductID = (uint)product.ProductID
                          })
                        )
                      .OrderByDescending(x => x.Score)
                      .Take(3)
                      .ToArray();

                    /*
                        Öneriler id ve skor duran standart output nesnesine düşer.
                        Ürün bilgisini de buraya katmak istediğimizden 
                        Recommendation sınıfından türeyen EnrichedRecommendation isimli bir sınıf daha var.
                        Herbir ürün için bu öneriler oluşur ama...
                    */
                    model.Recommendations.AddRange(topThree
                      .Select(rec => new EnrichedRecommendation
                      {
                          RelatedProductID = rec.RelatedProductID,
                          Score = rec.Score,
                          ProductName = _db.Products.Find((int)rec.RelatedProductID).ProductName
                      }));
                }

                // ...ama tüm önerilerden en iyi üçü gereklidir. O nedenle son listeden tekrar top 3 yapılmış durumda
                model.Recommendations = model.Recommendations
                  .OrderByDescending(rec => rec.Score)
                  .Take(3)
                  .ToList();
            }

            return View(model);
        }
    }
}
