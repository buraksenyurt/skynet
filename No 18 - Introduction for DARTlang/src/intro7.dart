/*

  Dart dilinde Concurrency konusu da desteklenir.
  Yani birden çok işin eş zamanlı olarak başlatılması sağlanabilir.
  Üstelik her biri gerçekten de kendi bellek bölgesinde ve thread'i içinde çalışır.
  Yani tam bağımsız çalışan işçilerdir. Aralarında mesajlaşarak haberleşebilirler. Bu açıdan Javascrip tarafındaki Web Worker'lara benzetilirler.

  Bunun için isolate kütüphanesindeki Isolate sınıfı kullanılır.

  Örneği üstüste birkaç kez çalıştırmakta yarar var. Çağırılan metot sıraları farklılık gösterecektir. 
  Isolate metotlarını her zaman aynı sırada başlatmaz.

*/

import 'dart:io';
import 'dart:isolate';

main(List<String> args) {
  //spawn ile eş zamanlı çalışacak 3 fonksiyon çağrısı tanımlandı.
  // spawn Function<T> tipinden parametreler alır.
  // Geriye Future döndürür. Doğal olarak asenkron yapıdadır.
  // Spawn static veya top level fonksiyonları işaret edebilir
  Isolate.spawn(Worker.CalculateTime, null);
  Isolate.spawn(Worker.GetPlayerStatistics, "Jordan");
  Isolate.spawn(Worker.TrashGarbage, null);

  print("İşler tetiklendi");
  sleep(Duration(seconds: 3)); // Ekran kapanmadan diğer işler bitsin diye eklendi.
}

class Worker {
  // static metotlar çağırılırken tanımlandığını sınıfa ait nesne örneği gerektirmez
  // arg değeri spawn metodunun ikinci parametresi olarak gelir. Genellikle concurrent operasyonu bir nesne taşımak için kullanılır.
  static void GetPlayerStatistics(var arg) {
    print("${arg} için istatistikler çekiliyor");
    print("Oyuncu istatistikleri hazır");
  }

  static void CalculateTime(var arg) {
    print("Zaman değerleri hesaplanıyor");
    print("Hesaplandı");
  }

  static void TrashGarbage(var arg) {
    print("Atıl nesneler atılıyor");
    print("Atıldılar...");
  }
}
