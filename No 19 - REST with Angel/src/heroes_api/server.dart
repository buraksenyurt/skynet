// Gerekli kütüphaneleri import ediyoruz

import 'dart:convert';
import 'dart:io';
import 'package:angel_framework/angel_framework.dart';
import 'package:angel_framework/http.dart';

main() async {
  var app = Angel();
  var http = AngelHttp(app);

  // /ping adresine bir get talebi gelirse döneceğimiz cevap
  // req=request nesnesi(istemciden gelen talebe ait içeriği yakalayabileceğimiz nesne), res=response nesnesi(istemciye cevap dönerken kullanılan nesne)
  app.get(
      "/ping", (req, res) => res.write("PONG! :)")); //Düz Text içeriği dönecek

  // route üstünden gelen parametrelerin nasıl kullanıldığını gösterir
  // önce json dosya içeriğini Hero tipinden bir listeye alır
  // Sonrasında name ile gelen kahraman bu listede aranır. Bulunursa geri döndürülür
  app.get('/heroes/help/:name', (req, res) async {
    var heroName = req.params["name"];
    List<Hero> list;
    print("$heroName çağırıldı");

    await new File('superHeroes.json').readAsString().then((fileContent) {
      // dosya içeriğini json nesnesine dönüştürüyoruz
      dynamic decoded = json.decode(fileContent);
      //Json nesnesini Map verisine dönüştür
      final converted = decoded.cast<Map<String, dynamic>>();

      list = converted
          .map<Hero>((json) => Hero.fromJson(json))
          .toList(); // Map verisinden ve Hero sınıfının fromJson metodundan yararlanarak içeriği nesne listesine döndür
      // print(list.length); // kaç eleman yüklendiğine bakarak bir kontrol gerçekleştirdim

      // LINQ benzeri sorguyla kahramanı bu listede isminden bulmaya çalıştım
      final findingHero =
          list.where((element) => element.super_hero == heroName).first;
      // print(findingHero.alter_ego); // Kontrol amaçlı

      if (findingHero != null) {
        // Bulunan nesne örneğini JSON' a dönüştürüyor
        var json = jsonEncode(findingHero.toJson());

        res.headers["content-type"] = "application/json";
        res.write(json);
      } else {
        throw AngelHttpException.badRequest(
            message:
                'Aradığın kahramanı ne yazık ki bulamadım... :('); //HTTP 400 Bad Request dönüyoruz
      }
    });
  });

  // basit bir post talebini nasıl ele alırız
  app.post('/messages/new', (req, res) async {
    await req.parseBody(); //HTTP talebindeki içeriği yakalıyoruz

    //içinden myMessage isimli niteliği alıyoruz
    var msg = req.bodyAsMap['myMessage'] as String;

    if (msg == null) {
      //Olur ya gövdede myMessage diye bir attribute yoktur
      //O zaman HTTP 400 Bad Request dönüyoruz
      throw AngelHttpException.badRequest(message: 'Mesajını göremedim :(');
    } else {
      //Bak yine tekrar ettim. Tüm uygulamanın JSON döndüreceğini nasıl söyleyebilirim acaba?
      res.headers["content-type"] = "application/json";
      res.write('{"yourMessage":"$msg"}');
    }
  });

  await http.startServer('localhost', 4444).then((value) => print(
      "Sunucu dinlemede")); // HTTP Server localhost:4444 üzerinde dinlemede olacak
}

// SuperHeroes dosyasındaki JSON içeriğini kod tarafında karşılayacak olan sınıf
class Hero {
  String super_hero, publisher, alter_ego, first_appearance, characters;
  Hero(
      {this.super_hero,
      this.publisher,
      this.alter_ego,
      this.first_appearance,
      this.characters});

  // JSON'dan nesne örneğine parse sırasında devreye girecek
  // Map veri tipi parametre olarak geliyor ve json içeriğinden eşleştirilen niteliklerle değerlerini taşıyor
  // Metodun yine Hero tipinden(kendi tipinden) bir örnek döndürdüğü dikkatinizi çekti mi?
  factory Hero.fromJson(Map<String, dynamic> json) {
    return Hero(
        //tek tek json içerikleri sınıf alanlarına atanıyor. Atanırken tip dönüşümü yapılıyor
        super_hero: json['superhero'] as String,
        publisher: json['publisher'] as String,
        alter_ego: json['alter_ego'] as String,
        first_appearance: json['first_appearance'] as String,
        characters: json['characters'] as String);
  }

  // Bu da Hero nesne içeriğini JSON tipine dönüştüren metot
  Map<String, dynamic> toJson() => {
        'superhero': super_hero,
        'publisher': publisher,
        'alter_ego': alter_ego,
        'first_appearance': first_appearance,
        'characters': characters
      };
}
