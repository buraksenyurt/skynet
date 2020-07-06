using System.Collections.Generic;

namespace NorthwindLib
{
    public class Company{
        public int CompanyID { get; set; }
        public string Name{get;set;}
        public string Description { get; set; }
        public ICollection<Game> Games { get; set; }
    }

}