using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;

namespace RewardService.Controllers
{
    [ApiController]
    [Route("[controller]")]
    public class CalculatorController : ControllerBase
    {
        private static readonly string[] topics = new[]
        {
            "1000 Free Spell"
            , "10 Free Coin"
            , "30 Days Free Trail"
            , "Gold Ticket"
            ,"Legendary Tournemant Pass"
            ,"1000 Free Retro Game"
            ,"One Day All Games Free"
        };

        private readonly ILogger<CalculatorController> _logger;

        public CalculatorController(ILogger<CalculatorController> logger)
        {
            _logger = logger;
        }

        [HttpGet]
        public IEnumerable<Reward> Get()
        {
            var rng = new Random();
            return Enumerable.Range(1, 3).Select(index => new Reward
            {
                Duration = rng.Next(7, 60),
                Description = topics[rng.Next(topics.Length)]
            })
            .ToArray();
        }
    }

    public class Reward{
        public int Duration { get; set; }
        public string Description { get; set; }
    }
}
