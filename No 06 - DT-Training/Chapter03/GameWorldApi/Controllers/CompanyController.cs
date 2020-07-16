using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;
using GameWorldApi.Repository; //Eklendi
using NorthwindLib;//Eklendi

namespace GameWorldApi.Controllers
{
    [ApiController]
    [Route("api/[controller]")]
    public class CompanyController : ControllerBase
    {

        private readonly ILogger<CompanyController> _logger;
        private ICompanyRepository _repository;
        /*
            Startup sınıfındaki ConfigureServices metodunda hatırlanacağı üzer ICompanyRepository için bir kayıt işlemi yapılmıştı.
            Oradaki tanıma göre çalışma zamanında buraya bir CompanyRepository nesnesi gelecek.
        */
        public CompanyController(ILogger<CompanyController> logger, ICompanyRepository repository)
        {
            _logger = logger;
            _repository = repository;
        }

        // Route(/) için HTTP Get talebi gelirse
        [HttpGet]
        public async Task<IEnumerable<Company>> GetAllCompanies() => await _repository.GetAllAsync();

        // HTTP Get talebinde firma için id değeri yollanırsa
        [HttpGet("{id}")]
        public async Task<IActionResult> GetCompany(int id)
        {
            var company = await _repository.GetAsync(id);
            if (company != null)
            {
                return Ok(company); // Aranan firma bulunursa HTTP 200 ile birlikte bilgileri döndürülür.
            }
            else
            {
                return NotFound(); //Arana firma bulunamadıysa HTTP 404 Not Found mesajı döndürürlür
            }
        }
    }
}
