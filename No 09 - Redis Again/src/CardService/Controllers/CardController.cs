using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;
using GameModel;

namespace CardService.Controllers
{
    [ApiController]
    [Route("api/[controller]")]
    public class CardController : ControllerBase
    {
        private readonly ILogger<CardController> _logger;

        public CardController(ILogger<CardController> logger)
        {
            _logger = logger;
        }

        [HttpGet("{heroName}")]
        public ActionResult<List<Card>> Get(string heroName)
        {
            // Kobay veri seti. Normal şartlarda gelen heroName parametresine göre bir veri kaynağından sorguladığımız kartları getirmemiz lazım
            // Şimdilik Redis denediğim GameCore servisine hizmet edecek
            // Bu arada bunlar Hearthstone kahraman ve kartları
            return new List<Card>{
                new Card{
                    CardID=1092,
                    Name="Argent Protector",
                    Spell=2,
                    Attack=2,
                    Health=2,
                    Description="Battlecry: Give a friendly minion divine shield.",
                    Hero=new Hero{HeroID=1, Name="Paladin", Description="Paladin hero description"}
                },
                new Card{
                    CardID=1029,
                    Name="Shattered Sun Cleric",
                    Spell=3,
                    Attack=3,
                    Health=2,
                    Description="Give a friendly minion +1/+1",
                    Hero=new Hero{HeroID=1, Name="Paladin", Description="Paladin hero description"}
                },
                new Card{
                    CardID=8923,
                    Name="Blessing of Kings",
                    Spell=4,
                    Attack=0,
                    Health=0,
                    Description="Give a friendly minion +4/+4",
                    Hero=new Hero{HeroID=1, Name="Paladin", Description="Paladin hero description"}
                }
            };
        }
    }
}
