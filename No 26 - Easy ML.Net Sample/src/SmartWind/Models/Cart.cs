using System.Collections.Generic;

namespace SmartWind.Models
{
    public class Cart
    {
        public IEnumerable<CartItem> Items { get; set; }
    }
}