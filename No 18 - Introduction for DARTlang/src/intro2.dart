class Vehicle {
  int x;
  int y;
  bool engineIsOn;
  String name;

  // Yine yapıcı metot tanımımız var.
  // Bu sefer opsiyonel parametre de kullanıyoruz.
  // Böylece varsayılan yapıcı metot dahil toplamda dört versiyonla nesne örneği oluşturabiliriz
  Vehicle(
      {this.name = "anonymous",
      this.x = 0,
      this.y = 10,
      this.engineIsOn = false});

  // Kolaylık olsun diye toString metodunun varsayılan davranışını ezdik
  @override
  String toString() => "$name ($x:$y) - $engineIsOn";
}

void main(List<String> args) {
  var ghost = Vehicle(); //parametresiz constructor çağırılır
  print(ghost);

  var leopard = Vehicle(
      name: "leopard",
      x: 15,
      y: 20,
      engineIsOn: true); // opsiyonel parametre değerleri atanmalıdır
  print(leopard);

  var tiger = Vehicle(
      y: -45,
      name:
          "tiger t-10"); // sadece name ve y değişkenine değer atadığımız durum
  print(tiger);
}
