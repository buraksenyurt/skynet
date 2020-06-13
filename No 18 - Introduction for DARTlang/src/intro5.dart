/* 
DART, fonksiyonel dil özelliklerini de bünyesinde barındırır.

Yani, fonksiyonları başka fonksiyonlara parametre olarak geçebilir, fonksiyonları değişkenlere atayabilir, isimsiz fonksiyonlar (anonymous'u hatırlayalım) yazabilir,
currying yapabiliriz (çok parametreli bir fonksiyonu tek parametreli hale getirip kullandırtmak)...

Bu örnekte birkaç fonksiyonel yaklaşımın nasıl kullanıldığına yer verilmektedir.
*/

import 'dart:math'; //Matematik kütüphanesinden karekök fonksiyonunu kullanmak için

main(List<String> args) {
  // kobay bir sayı listesi
  var numbers = [4, 2, 9, 8, 6, 7, 6, 5, 1, 3, 3, 1, 6, 8];

  // Mesela 3ncü ile 8nci arasındakilerinde bir işlem yapıp bunu imperative yaklaşımla ekrana yazdırmak istesek
  for (var i = 3; i < 8; i++) {
    print(sqrt(numbers[i]));
  }

  print("----");
  // Şimdi yukarıdaki işlemi fonksiyonel yaklaşımla yazalım
  numbers.skip(3).take(5).map(sqrt).forEach(print);

  // Birkaç fat-arrow function tanımlaması
  var sum = (x, y) => x + y;
  print(sum(3, 4));

  var multi = (num x, num y) => x + y; // bu sefer parametre tipi belirttik
  print(multi(3.56, -4.213));

  num div(num x, num y) =>
      x / y; // Hem dönüş hep parametre tipleri açıkça belirtiliyor
  print(div(3, 9));

  // Böyle güzel şeyler de yazabiliyoruz. Parametre olarak gelen
  // herhangi bir String listenin elemanlarının karakter sayısının toplamını bulan fonkisyon
  num findCharCount(List<String> names) =>
      names.map((name) => name.length).fold(0, (num a, num b) => a + b);
  print(findCharCount(["bir", "iki", "üç", "dört"]));
  print(findCharCount(
      ["black", "window", "currying", "livinsteineinenkovskiyeviç"]));
}
