/*
  Dart asenkron programlamada Stream adı verilen bir mevzu da var.
  Stream ile asenkron olarak çalışan olaylarda araya girebiliyoruz.
  Stream'ler verinin bir noktadan diğerine akarken kullanılan kanalı referans ediyorlar.
  Bu alana girerek akan veri üzerinde çeşitli işlemler yapabiliriz.

  Bu arada iki tür Stream var. Broadcast ve Single Subscription ki henüz ne olduklarını tam olarak öğrenemedim :(
*/

import 'dart:async';
import 'dart:io';

main() async {
  //awaitable çağrım içeriyor
  var stream = calculate(5); //stream nesnesini alıyoruz
  var total = await lookInsideStream(stream);
  print("\nToplam : ${total}");
}

// Asenkron çalışan bir operasyon ama geriye iteratif bir Stream döndürüyor.
// lookAtTheStream metodu bu dönen stream'i kullanıyor
Stream<num> calculate(num max) async* {
  for (int i = 1; i <= max; i++) {
    stdout.write("yield ${i}...");
    yield i;
  }
}

// Parametre olarak calculate çağrısı sonucu üretilen Stream'i almakta
// Buna göre kanal(channel) üstündeki her çağrımda devreye girecek.
Future<num> lookInsideStream(Stream<num> stream) async {
  var sum = 0;
  await for (var value in stream) { //kanaldaki verilerde hareket edebiliriz
    stdout.write(" (${value}) "); // o an yakaladığımız veri üstünde istediğimiz işlemi yapmamız mümkün
    sum += value;
  }
  return sum;
}
