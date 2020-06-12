// Basit bir sınıf tanımıyla işe başlayalım
class Player {
  // birkaç sınıf değişkeni. İlk etapta integer, string ve bool kullanabildim.
  // ek olarak double, list gibi veri türleri de var.
  int id;
  String nickName;
  int level;

  // burada read-only bir instance variable (bana göre özellikl) tanımı var
  bool _online = false;
  bool get online => _online; // sadece getter

  // Constructor tanımı. this ile doğrudan iç değişkenlere değer aktarımı sağlanır
  Player(this.id, this.nickName, this.level);
  
  // Yukarıdaki constructor bu şekilde de tanımlanabilir
  /*
    Player(id, nickName, level) {
    this.id = id;
    this.nickName = nickName;
    this.level = level;
  }
  */

  // toString metodunun ezilişi. toString çağrılabilen yerlerde bizim istediğim şekilde dönüş olacak
  @override
  String toString() =>
      "$id, $nickName, $level"; // fat-arrow notation'ına göre tek satırda fonksiyon tanımı

  // değer döndürmeyen bir iki metot
  void getOnline() {
    // if possible
    if (!_online) _online = true;
  }

  void getOffline() {
    if (_online) _online = false;
  }

  // String döndüren bir metot
  String getState() {
    if (!online)
      return "Active";
    else
      return "Passive";
  }
}

// Programın giriş noktası (entry point)
void main() {
  // var wilmort=Player();

  // Player tipinden bir nesne örneği tanımı. new operatörü de kullanılabilir ama şart değil.
  var runi = Player(1, "Ruynildson", 125);
  
  // runi'nin toString metodunu override etmiştik.
  print(runi);

  // Ternary operatörünün olmadığı bir programlama dili var mı acaba? :D
  print("Current state is ${runi.online == true ? "Active" : "Passive"}");

  runi.getOnline();
  print(runi.getState());

  runi.getOffline();
  print(runi.getState());

  // runi.online=false; // instance variable read-only tanımlandığı için burada derleme hatası oluşur
}