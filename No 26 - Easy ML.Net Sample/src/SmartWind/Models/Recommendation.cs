namespace SmartWind.Models
{
    /*
        Makine öğrenme algoritmasının çıktısı olan entity modeli.
        Önerilen ürün numarası ile skor puanını tutmakta.
    */
    public class Recommendation
    {
        public uint RelatedProductID { get; set; }
        public float Score { get; set; }
    }
}