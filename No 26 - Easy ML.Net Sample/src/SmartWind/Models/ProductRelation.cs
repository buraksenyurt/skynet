using Microsoft.ML.Data;

namespace SmartWind.Models
{
    /*
        Satın alınan bir ürünle ilintili diğer ürünlerin ilişkilendiği entity modeli.
        Bu aslında Matrix Factorization algoritmasının girdisi olan veriyi tutacak nesne.
    */
    public class ProductRelation
    {
        // 200 ile olası maksimum ID değerini belirttik. Küçük bir veri setinden çalışalım diye
        [KeyType(200)] // Column
        public uint ProductID { get; set; }
        [KeyType(200)] // Row
        public uint RelatedProductID { get; set; }
    }
}