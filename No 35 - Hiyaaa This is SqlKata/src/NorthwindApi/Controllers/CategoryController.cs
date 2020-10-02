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
            Insert metodu da kullanılabilir. InsertGetId, tabloda üretilen primary key değerini geriye veriyor.
        */
        [HttpPost]
        public IActionResult GetCustomerCountsByCity(Category category)
        {
            var inserted_id = _queryFactory
                .Query("categories")
                .Insert(new {
                    category_name=category.Name,
                    description=category.Description
                });

            return Ok(inserted_id);
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