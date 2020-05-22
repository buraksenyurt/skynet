using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;

namespace Readers.Controllers
{
    [ApiController]
    [Route("[controller]")]
    public class BookController : ControllerBase
    {
        private readonly ILogger<BookController> _logger;

        public BookController(ILogger<BookController> logger)
        {
            _logger = logger;
        }

        [HttpGet]
        public Book Get()
        {
            _logger.LogInformation("{date} zamanı itibariyle bir kitap önerildi...", DateTime.UtcNow);

            _logger.LogError(new System.IO.FileNotFoundException(), "Yeni bir kitap eklemeye çalışırken hata oluştu");

            return new Book { Name = "Yabancı", Authors = "Alber Camus" };
        }
    }
}
