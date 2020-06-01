using System;
using System.Collections.Generic;

namespace NorthwindApi.Models
{
    public class Category
        : Entity // Böyece hem Entity'ten gelen özellikleri taşıyacak hem de IEntity üzerinden repository tipinde kullanılabilecek
    {
        public string Name { get; set; }
        public string Description { get; set; }
        public ICollection<Product> Products { get; set; }
    }
}