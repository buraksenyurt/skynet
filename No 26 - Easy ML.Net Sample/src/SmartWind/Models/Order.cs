using System;
using System.Collections.Generic;

namespace SmartWind.Models
{
    public class Order
    {
        public int OrderID { get; set; }
        public string CustomerID { get; set; }
        public int EmployeeID { get; set; }
        public DateTime? OrderDate { get; set; }
        public DateTime? RequiredDate { get; set; }
        public DateTime? ShippedDate { get; set; }
        public int ShipVia { get; set; }
        public decimal? Freight { get; set; } = 0;
        public ICollection<OrderDetail> OrderDetails { get; set; }
        public Customer Customer { get; set; }
    }
}