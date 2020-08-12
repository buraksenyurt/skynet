using System.Collections.Generic;

/*
    Verisetlerinin eğitilip eğitilmediği bilgisini tutan Index modelimiz
*/
namespace SmartWind.Models
{
    public class HomeIndexViewModel{
        public IEnumerable<Category> Categories { get; set; }
        public bool UKDatasetExists { get; set; }
        public bool GermanyDatasetExists { get; set; }
        public bool USADatasetExists { get; set; }
        public long Milliseconds { get; set; }
    }
}