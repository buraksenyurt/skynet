using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;

namespace PromotionService.Controllers
{
    [ApiController]
    [Route("[controller]")]
    public class ApplierController : ControllerBase
    {
        private readonly ILogger<ApplierController> _logger;

        public ApplierController(ILogger<ApplierController> logger)
        {
            _logger = logger;
        }

        /*
            Birde HTTP Post deneyelim bari.
            Sembolik olarak promosyon uygulayan bir metot olduğunu varsayalım.
        */
        [HttpPost]
        public IActionResult SetPromotoion(Code promoCode)
        {
            return Ok($"{promoCode.No} için {promoCode.Duration} gün süreli promosyon kullanıcı hesabına tanımlanmıştır");
        }
    }

    public class Code
    {
        public string No { get; set; }
        public int Duration { get; set; }
        public int PlayerId { get; set; }
        public int GameId { get; set; }
    }
}
