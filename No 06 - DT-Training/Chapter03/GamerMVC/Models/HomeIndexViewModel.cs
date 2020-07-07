using System.Collections;
using System.Collections.Generic;
using NorthwindLib;

namespace GamerMVC.Models
{
    public class HomeIndexViewModel
    {
        public List<Company> Companies { get; set; }
        public List<Game> Games { get; set; }
    }
}