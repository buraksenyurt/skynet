/*
  Dart dilinde de asenkron fonksiyon kullanımları mümkündür. 
  Özellikle uzun süren işlerin ana iş parçacığını bekletmesini istemediğimiz durumlarda işe yarar.

  future bu anlamda önemli bir terimdir. Asenkron tasarlanan bir fonksiyon
  anında bir future döndürür. O anda feature Uncompleted pozisyonundadır.
  Asenkron çağırılan fonksiyon başarılı şekilde tamamlandığında future,
  Completed pozisyonuna geçer. Exception oluşursa da Completed pozisyonda kalamaz tabii.

  future dediğimiz şey esasında Future sınıfının bir nesne örneğidir.

  Asenkron bir fonksiyon değer dönebilir veya void olarak tanımlanabilir. 
  Future<void> ve Future<int> gibi.

  Declerative yaklaşıma göre asenkron olan fonksiyonlarda async kelimesi kullanılır.
  Asenkron fonksiyonun işini bitirdikten sonra sonucunu almak için await kullanılır.

*/
import 'reporter.dart';

main(List<String> args) async {
  var watson = Reporter();
  Counting(10);
  var value = await watson
      .GetReportResult(); //işlemlerin tamamlanmasını await ile bekletiyoruz
  print("Hesaplamalara göre risk değeri ${value}");
}

void Counting(int max) {
  for (var i = 1; i <= max; i++) {
    Future.delayed(Duration(seconds: i), () => print(i));
  }
}
