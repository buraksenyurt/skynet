using System.Collections.Generic;

namespace NorthwindLib
{
    public class Category{
        public int CategoryID { get; set; }
        public string CategoryName{get;set;}
        public string Description { get; set; }
        public ICollection<Product> Products { get; set; }
    }

}