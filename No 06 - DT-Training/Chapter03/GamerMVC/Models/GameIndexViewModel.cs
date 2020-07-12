using System.Collections.Generic;
using NorthwindLib;

namespace GamerMVC.Models
{
    public class GameIndexViewModel
    {
        public List<Game> Games { get; set; }
        public List<short?> Years { get; set; }
    }
}