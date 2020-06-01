using System;

namespace NorthwindApi.Models
{
    public class Product
        : Entity 
    {
        public string Title { get; set; }
        public decimal UnitPrice { get; set; }
        public int UnitsInStock { get; set; }
        public Category Category { get; set; }
    }
}