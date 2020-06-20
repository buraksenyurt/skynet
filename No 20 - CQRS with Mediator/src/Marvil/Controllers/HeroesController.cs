using System.Collections.Generic;
using System.Threading.Tasks;
using Marvil.Model;
using MediatR;
using Microsoft.AspNetCore.Mvc;
using Marvil.Handler;

namespace API.Controllers
{
    [Route("api/[controller]")]
    [ApiController]
    public class HeroesController : ControllerBase
    {
        // Mediator nesnesini constructor üzerinden enjekte ediyoruz
        private readonly IMediator _mediator;
        public HeroesController(IMediator mediator)
        {
            _mediator = mediator;
        }

        /*
            Yeni kahraman ekleme işinin ele alındığı metot. 
            Tipik HTTP Post.
            Parametre olarak Create tipi içindeki Command sınıfı (Handler görevini üstlenen) kullanılıyor
        */
        [HttpPost]
        public async Task<ActionResult<Unit>> Create(Create.Command command)
        {
            // Mediator gelen talebi uygun Handler'a yönlendirmekte
            return await _mediator.Send(command);
        }

        [HttpDelete("{name}")]
        public async Task<ActionResult<Unit>> Delete(string name)
        {
            return await _mediator.Send(new Delete.Command { Name = name });
        }

        // Veri güncelleme işini üstlenen operasyon
        [HttpPut]
        public async Task<ActionResult<Unit>> Update(Update.Command command)
        {
            return await _mediator.Send(command);
        }

        // Listeleme için gelen Http Get talebine karşılık çalışacak
        // Geriye Hero listesi döndürür
        [HttpGet]
        public async Task<ActionResult<List<Hero>>> List()
        {
            // İşte en güzel kısım :)
            // Listeleme davranışı için devreye giren Mediator nesnesi
            // Bu isteği List Handler içerisindeki Query sınıfına yönlendiriyor.
            // Diğer metotlarda da sadece Send fonksiyonunu çağırdığımıza ve gerekli Query ya da Command nesnesini parametre olarak verdiğimize dikkat edelim.
            return await _mediator.Send(new List.Query());
        }

        // İsimden kahraman detaylarını döndüren HTTP Get operasyonu
        [HttpGet("{name}")]
        public async Task<ActionResult<Hero>> Single(string name)
        {
            /*
                Single sınıfı içindeki Query nesnesini örneklerken,
                gerekli isim parametresini de besliyoruz
            */
            return await _mediator.Send(new Single.Query() { Name = name });
        }

        [HttpGet("gt/{value}")]
        public async Task<ActionResult<List<Hero>>> GreaterThan(int value)
        {
            return await _mediator.Send(new GreaterThan.Query() { LevelValue = value });
        }
    }
}