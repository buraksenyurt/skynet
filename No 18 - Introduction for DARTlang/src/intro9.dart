/*

  Bu sefer ki örnekte ilkel bir web server nasıl yazılabilir onu anlamaya çalışacağız.
  Daha önceki örneklerde gördüğümüz async, await, Future ve hatta concurrent ile ilişkili olan
  Stream'ler burada daha anlamlı hale geliyor.

  Aşağıdaki örnek gelen HTTP Get taleplerinin hepsine standart bir HTML içeriği döndürüyor.
  Harici taleplerde ise (HTTP Post,Put,Delete gibi) MethodNotAllowed (HTTP 405) scevabı verilmekte.
*/

import 'dart:io';

Future main(List<String> args) async {
  // localhost:8887 nolu adresi kendisine bağlayarak sunucu nesnesini oluşturduk
  // loopbackIPv4 localhost veya 127.0.0.1'i işaret edecektir. Portu tamamen keyfi seçtik.
  var server = await HttpServer.bind(
      InternetAddress.loopbackIPv4, 8887); //awaitable bir çağrıdır

  print("Sunucu ${server.port} üstünden dinlemede");

  // server üzerine gelen http talepleri eş zamanlı olarak dinleniyor
  // HttpServer sınıfı Stream'leri kullandığından aşağıdaki gibi await for yazarak gelen her talep sonrası araya girmek mümkün oluyor.
  await for (HttpRequest request in server) {
    // Taleple ilgili birkaç bilgi yazdırabiliriz. HTTP Metodu, gelen talep adresi vs...
    print("Gelen talep, ${request.method}\n Uri : ${request.requestedUri}");

    if (request.method == "GET") {
      // Gelen talep Get metodu ise bunları bunları yap
      // .. ile response nesnesi üzerinden aynı ifadede hem özellik değeri atayabilir hem de metot çağrısı gerçekleştirebiliriz
      request.response
        ..statusCode = HttpStatus.ok // HTTP Statü kodu olarak 200 dönüyoruz
        ..headers.contentType = ContentType
            .html // Döndürdüğümüz içeriğin HTML formatında olduğu Header ile belirtiyoruz
        ..write(
            "<h2>Sunucu zamanı : ${DateTime.now().toString()}</h2>") // basit bir HTML içerik döndürüyoruz
        ..write(
            "<p>Şu anda ${request.requestedUri.path} adresine talepte bulundunuz</p>")
        ..close(); // response diğer dillerde olduğu gibi kapatılmalı
    } else {
      //değilse istemciye söz konusu metot çağrısına izin verilmediğini söyle
      request.response
        ..statusCode = HttpStatus.methodNotAllowed
        ..write(
            "${request.method} metodu bu sunucu tarafından desteklenmemektedir.")
        ..close();
    }
  }
}
