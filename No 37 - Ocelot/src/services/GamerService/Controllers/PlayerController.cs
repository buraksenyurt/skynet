using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;

namespace GamerService.Controllers
{
    [ApiController]
    [Route("[controller]")]
    public class PlayerController : ControllerBase
    {
        private readonly ILogger<PlayerController> _logger;

        public PlayerController(ILogger<PlayerController> logger)
        {
            _logger = logger;
        }

        /*
            HTTP Get taleplerine karşılık verecek metodumuzdan geriye sembolik olarak bir Player nesnesi döndürüyoruz.
            Player/19 gibi gelen taleplere cevap verecek
        */
        [HttpGet("{id}")]
        public Player Get(string id)
        {
            return new Player
            {
                Id = id,
                Fullname = "Megen Enever",
                Level = 58,
                Location = "Dublin"
            };
        }
    }

    public class Player
    {
        public string Id { get; set; }
        public string Fullname { get; set; }
        public int Level { get; set; }
        public string Location { get; set; }
    }
}
