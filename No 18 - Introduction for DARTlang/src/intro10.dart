/*
  Web server maceralarına devam.
  Bu seferki örnekte fiziki bir JSON dosyasının içeriğini geriye döndürüyoruz.
  Bunu, sadece HTTP Get metodunda ve /heros path'ine gelen talepler karşılığında yapıyoruz.
  Buradan yola çıkarak çok basit bir CRUD Rest servisine kadar gidilebilir.
*/
import 'dart:io';

Future main(List<String> args) async {
  // localhost ve 8888 portunu server nesnesine bağladık
  var server = await HttpServer.bind(InternetAddress.loopbackIPv4, 8888);

  print("${server.port} nolu portan dinlemedeyiz");

  // server ile açılan stream üstünde gerçekleşen istekleri dinliyoruz
  await for (HttpRequest req in server) {
    print("${req.method}, ${req.uri}");

    // Eğer talep GET metodundaysa ve path /heros ise
    if (req.method == "GET" && req.requestedUri.path == "/heroes") {
      // JSON formatında içerik döneceğimiz için ContentType'a uygun değeri atadık
      req.response.headers.contentType = ContentType.json;
      // Statü kodunu güncelledik
      req.response.statusCode = HttpStatus.ok;
      // JSON verisi için kaynak -> https://gist.github.com/mariodev12/a923f2b651a005ca3ca7f851141efcbc

      // İlginç bir kod parçası değil mi?
      // File nesnesini bir değişkene atamadan kullanıyoruz.
      await new File('superHeroes.json') // json dosyasının içeriğini
          .readAsString() //string formatta oku
          .then((content) => req.response.write(
              content)); //okuma tamamlandığında response üstüne yaz (Zaten saf json içeriği olduğundan decode etmemiz gerek yok)

    } else {
      // Farklı bir talep gelirse 404 Not Found muamelesi göster
      req.response.statusCode = HttpStatus.notFound;
      // Nezaketen de bir cevap yaz :D
      req.response.write(
          "${req.method} ve ${req.requestedUri.path} kullanılabilir değil.");
    }
    await req.response
        .close(); // buffer'a alınmış bir şeyler olma ihtimaline karşın tüm içeriği yazdırmayı garantile
  }
}
