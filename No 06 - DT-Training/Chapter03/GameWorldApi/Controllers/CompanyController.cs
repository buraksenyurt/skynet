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

        // HTTP Post talebi gelir ve gövdede JSON formatında Company içeriği olursa ekleme işlemini gerçekleştiririz
        [HttpPost]
        public async Task<IActionResult> AddCompany([FromBody] Company company)
        {
            if (company == null)
            {
                return BadRequest();
            }
            var added = await _repository.CreateCompanyAsync(company);
            return Ok(added);
        }

        // Silme operasyonumuz. HTTP Delete talebi gelirse, ID değeri üstünden gerçekleştirilir
        [HttpDelete("{id}")]
        public async Task<IActionResult> Remove(int id)
        {
            var founded = await _repository.GetAsync(id);
            if (founded == null)
            {
                return NotFound();
            }
            bool? deleted = await _repository.DeleteAsync(id);
            if (deleted.HasValue && deleted.Value)
            {
                return new NoContentResult();
            }
            else
            {
                return BadRequest($"{id} numaralı bir firma bulunamadığı için silme işlemi gerçekleşmedi.");
            }
        }

        //TODO Update operasyonu yazılıp test edilmeli
    }
}
