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
    public class CustomerController : ControllerBase
    {
        private readonly ILogger<CustomerController> _logger;
        private readonly QueryFactory _queryFactory;

        public CustomerController(ILogger<CustomerController> logger, QueryFactory queryFactory)
        {
            _logger = logger;
            _queryFactory = queryFactory;
        }


        /*
            Hangi şehirde kaç müşterimiz olduğunu döndüren action.
            customers tablosunda city bilgisine göre gruplama yapıp, count alıyoruz yani.

            Örnek sorgu -> https://localhost:5001/api/customer/cityreport
        */
        [HttpGet("cityreport")]
        public IActionResult GetCustomerCountsByCity()
        {
            var report = _queryFactory
                .Query("customers")
                .Select("city")
                .SelectRaw("count(customer_id) as count")
                .GroupBy("city")
                .Get();

            return Ok(report);
        }
    }
}