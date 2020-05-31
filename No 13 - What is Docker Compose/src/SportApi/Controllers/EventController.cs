using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;

namespace SportApi.Controllers
{
    [ApiController]
    [Route("[controller]")]
    public class EventController : ControllerBase
    {
        private readonly ILogger<EventController> _logger;

        public EventController(ILogger<EventController> logger)
        {
            _logger = logger;
        }

        [HttpGet]
        public Event GetTodaysEvent()
        {
            return new Event{
                Date=DateTime.Now,
                Title="Günün sportif başarısı...",
                Description="Bugün olan sportif olayın başarısına ait bilgiler"
            };
        }
    }
}
