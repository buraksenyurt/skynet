using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;

/*
    SqlKata kullanımı için eklenen namespace bildirimleri
*/
using SqlKata;
using SqlKata.Execution;

namespace NorthwindApi.Controllers
{
    [ApiController]
    [Route("api/[controller]")]
    public class ProductController : ControllerBase
    {
        private readonly ILogger<ProductController> _logger;
        /*
            Startup tarafında bildirimini yaptığımız QueryFactory nesnesini
            constructor ile buraya enjekte ettik. Böylece controller içindeki
            tüm action metotlarında SqlKata yı kullanabileceğiz.
        */
        private readonly QueryFactory _queryFactory;

        public ProductController(ILogger<ProductController> logger, QueryFactory queryFactory)
        {
            _logger = logger;
            _queryFactory = queryFactory;
        }

        /*
            İlk SqlKata denemem.
            products tablosunda discontinued olanların listesini çekmeye çalışıyoruz
            Geriye JSON içerik dönecektir
        */
        [HttpGet("Discontinued/")]
        public IActionResult GetDiscontinuedProducts()
        {
            var products = _queryFactory
                .Query("products") // products tablosu için sorgu hazırlanacak
                .Select("product_id", "product_name", "unit_price") // sadece bu alanlar getirilecek
                .Where("discontinued", 1) // discontinued değeri 1 olanlar çekilecek
                .Get();

            //_logger.LogInformation($"{DateTime.UtcNow.ToLongTimeString()} - ProductController - GET");

            return Ok(products);
        }

        /*
            Kategorileri listeleyen action

            https://localhost:5001/api/product/categories/
        */
        [HttpGet("categories/")]
        public IActionResult GetCategories()
        {
            var categories = _queryFactory
                .Query("categories")
                .OrderBy("category_name")
                .Get();

            return Ok(categories);
        }
    }
}
