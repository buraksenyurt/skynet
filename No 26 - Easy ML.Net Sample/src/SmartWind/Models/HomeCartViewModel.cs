using System.Collections.Generic;

/*
    Önyüzde öneri ürünlere ait bilgileri gösterecek olan Model sınıfımız.
*/
namespace SmartWind.Models
{
    public class HomeCartViewModel
    {
        public Cart Cart { get; set; }

        public List<EnrichedRecommendation> Recommendations { get; set; }
    }
}