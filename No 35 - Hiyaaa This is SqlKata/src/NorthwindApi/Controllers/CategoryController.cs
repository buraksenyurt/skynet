using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;
using NorthwindApi.Model;

/*
    SqlKata kullanımı için eklenen namespace bildirimleri
*/
using SqlKata;
using SqlKata.Execution;

namespace NorthwindApi.Controllers
{
    [ApiController]
    [Route("api/[controller]")]
    public class CategoryController : ControllerBase
    {
        private readonly ILogger<CategoryController> _logger;
        private readonly QueryFactory _queryFactory;

        public CategoryController(ILogger<CategoryController> logger, QueryFactory queryFactory)
        {
            _logger = logger;
            _queryFactory = queryFactory;
        }


        /*
            Yeni bir kategori eklemek için kullanacağımız post action.
            Parametre olarak gelen JSON içeriğindeki alanları kullanıyor.
            Insert işlemi sonucuna göre de Ok veya 500 dönüyoruz.

            Adres : https://localhost:5001/api/category
            Metot : HTTP Post
            Body : 
            {
                "CategoryId":10,
                "Name": "Kitap",
                "Description": "Kitap konulu ürünler"
            }
        */
        [HttpPost]
        public IActionResult GetCustomerCountsByCity(Category category)
        {
            try
            {
                var inserted_id = _queryFactory
                                .Query("categories")
                                .Insert(new
                                {
                                    category_id = category.CategoryId,
                                    category_name = category.Name,
                                    description = category.Description
                                });
                return Ok(category);
            }
            catch (Exception exp)
            {
                _logger.LogError(exp.Message);
                return StatusCode(500, "Kategori ekleme işlemi başarısız!");
            }
        }

        /*
            Denemeler sırasında categories tablosunu kirletecek yeni satırlar ekledim tabii :)
            Silme operasyonu da lazım.

            Örnek sorgu https://localhost:5001/api/category/10
            Metot HTTP Delete
        */
        [HttpDelete("{id}")]
        public IActionResult Delete(int id)
        {
            int deleted = _queryFactory.Query("categories").Where("category_id", id).Delete();
            return Ok(deleted);
        }

        /*
            Kategorileri listeleyen action

            https://localhost:5001/api/category
        */
        [HttpGet]
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