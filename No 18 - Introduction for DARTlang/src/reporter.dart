class Reporter {
  // asenkron çağırılabilecek şekilde tasarlanan fonksiyonumuz. async ve await kullanımlarına dikkat.
  // await fonksiyonları sadece async fonksiyonlar içerisinde kullanılır
  Future<num> GetReportResult() async {
    print("Hesaplanıyor...");
    var value =
        await _calculateRisk(); // Uzun süren fonksiyone awaitable. O bitene kadar kodu duraksattık
    print("Hesaplandı");
    return value;
  }

  // Risk değeri hesap eden bütçe fonksiyonumuzun uzun sürdüğünü varsayalım
  // delayed ile 5 saniyelik suni bir gecikme yaratıyoruz
  // 5 saniye sonrasında ise fonksiyona geriye 0.17 değerini taşıyan bir future döndürüyor
  Future<num> _calculateRisk() => Future.delayed(
        Duration(seconds: 5),
        () => 0.17,
      );

  // Bu arada metot adındaki _ işareti onu private yapar. Yani bu library dışında erişilemez. Mesela intro6.dart içinden.
  // Yani bir library içindeki metodun ya da alanın private olması isteniyorsa adının başına _ işareti konur
}
