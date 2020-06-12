/*
Bu örnek data.dart isimli kütüphaneyi (library) kullanıyor.
data kütüphanesinde basit bir Abstract Factory deseni uyarlaması var.
Hem kendi kütüphanemizi nasıl kullanırız hem de abstract sınıftan türetme gibi OOP
özelliklerini nasıl uygularız diye bakıyoruz.
*/
import 'data.dart'; // Kendi yazdığımız kütüphane bildirimi

// List<String> şaşırtmasın. Komut satırından parametre alabiliyoruz
main(List<String> args) {
  var option = args[0]; //terminal penceresinden ilk parametreyi alıyoruz.

  // rapor üretici fabrika değişkeni tanımı
  ReportFactory factory;

  // bir seçime göre gerekli factory nesnesi örnekleniyor
  switch (option) {
    case "b":
      factory = new BudgetReportFactory();
      break;
    case "e":
      factory = new EmployeesReportFactory();
      break;
    case "p":
      factory = new PerformanceReportFactory();
      break;
    default:
      print("ne desem bilemedim");
  }

  // Asıl istediğimiz raporu ürettiriyoruz
  var report = factory.CreateReport();
  // ve hangi rapordan istediysek onun publish metodunun çıktısını ekranda görmeyi bekliyoruz
  report.publish();
}
