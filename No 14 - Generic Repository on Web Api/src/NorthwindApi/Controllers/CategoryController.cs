using System;
using System.Collections.Generic;
using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Authorization;
using NorthwindApi.Repositories;
using NorthwindApi.Models;
using System.Threading.Tasks;

namespace NorthwindApi.Controllers
{
    /*
    Standart controller sınıfımız

    */
    [ApiController]
    [Route("northwind/api/[controller]")]
    public class CategoryController
        : ControllerBase
    {
        private IRepository<Category> _repository;
        // repository AddScoped tanımı gereği CategoryRepository olarak gelecek
        public CategoryController(IRepository<Category> repository) => _repository = repository;

        [HttpGet]
        public async Task<IEnumerable<Category>> GetAll() => await _repository.ReadAllAsync();

        [HttpGet("{id}")]
        public async Task<IActionResult> GetById(int id)
        {
            // Örnek olması açısından kategori bulunamazsa
            // HTTP 404 Not Found dönüyoruz. Bulunursa HTTP 200 ile birlikte
            // bulunan kategoriyi döndürüyoruz.
            var founded = await _repository.ReadAsync(id);
            if (founded == null)
                return NotFound();

            return Ok(founded);
        }

        [HttpPost]
        public async Task Add([FromBody] Category category) => await _repository.CreateAsync(category);

        [HttpDelete("{id}")]
        public async Task<IActionResult> Delete(int id)
        {
            var result = await _repository.DeleteAsync(id);
            if (result == false)
                return NotFound();

            return Ok();
        }
    }
}