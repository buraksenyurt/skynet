/*
internal library kullanımı örneği.
dart.convert içerisinde JSON ve UTF8 için faydalı dönüştürme operasyonları var.
*/
import 'dart:convert';

main(List<String> args) {
  var someData = '''[
    {"id": 1, "color": "blue", "size": 10.50},
    {"id": 2, "color": "red", "size": 19},
    {"id": 3, "color": "green", "size": 50.987854}
  ]
  '''; // örnek bir JSON içeriği var.

  var decoded =
      jsonDecode(someData); // decode edip aşağıdaki gibi kullanabiliriz

  for (var d in decoded) {
    print(
        "${d['id']} , ${d['color']}, ${d['size']}"); // String Interpolation sağolsun
  }

  print(decoded[1][
      'color']); // decode edilmiş dizideki birinci elemanın color niteliğinin değerini yazdırır

  //print(decoded);
}
