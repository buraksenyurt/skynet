/*
    RUST dilinde Garbage Collector mekanizması yoktur.
    Ownership (Sahiplik) dilin en önemli olgularındandır.
    Belleğin Stack ve Heap alanlarının ne olduğunu iyi bilmeyi gerektirir.
*/

fn main() {
    /*
        Önce scope kavramına bir değinmek lazım.
        Aşağıda {} arasında bir scope açtık. Bu scope içinde tanımlı değişkenler sadece bu scope içinde kullanılabilir.
    */
    {
        // greetings değişkeni henüz scope'a dahil değil
        let greetings = "It's raining..."; // scope'a dahil oldu
        println!("{}", greetings); // scope içinde kullanıldı
    } //burası açtığımız scope'un sonlandığı yer
      // println!("{}", greetings); // greetings artık scope dışı ve kullanılamaz

    /*
        string demişken...
        Doğal olarak string literal ile tanımlanan değişkenler de diğer türler gibi varsayılan olarak immutable'dır.
        Diğer yandan string içeriği kullanıcı tarafından çalışma zamanında da girilebilir. Hatta bu belki bir dosyanın içeriğidir.
        Yani başlangıçta ne kadar alan kaplayacağı belli olmayabilir.
        String veri tipinden yararlanarak içeriği çalışma zamanında belli olacak metinsel içerikler tanımlayabiliriz.
        Bilin bakalım String türü bellekte nerede durur (Heap)
    */
    {
        // Yeni bir scope açtık
        let mut username = String::from("Jean"); //scope içinde geçerli
        username.push_str("Van Damme"); // metne yeni bilgi ekledik. username mutable hale getirildi.
        println!("{}", username); // scope içinde kullandık
    } //scope dışına çıktık. username kaynağa iade edildi
      // Scope dışına çıkıldığında Rust çalışma zamanı drop isimli bir fonksiyon çağırır. C#'taki Destructor gibi düşünebilirim.

    /*
        Değişkenler arası atamalar, bellekte tutuldukları lokasyonlara göre farklı davranışlar gösterirler.
        Stack'te tutulan sayısal değerler ile String'i karşılaştıralım.
        Özellikle String'lerin atamasında move adı verilen bir olay söz konusudur
    */
    let x = 10; // stack'de x için yer açıldı
    let mut y = x; // stack'de y için yer açıldı ve içine x'in değeri kopyalandı
    y += 5; // y değerini değiştirdim. Atayama rağmen bu x'in değerini bozmaz
    println!("x={} y={}", x, y);

    // Şimdi String tipinin durumuna bakalım
    // start_city değişkeni tanımlandığında stack'te bir işaretçi alan ve heap'te de içeriğin tutulduğu alanlar ayrılır
    // stack'te değişken heap'e referans ettiği adres bilgisi, içeriğin uzunluğu(Length) ve yine içeriğin byte cinsinden ne kadar alan tuttuğu(Capacity) bilgileri de yer alır
    let start_city = String::from("London");
    let end_city = start_city; // x ve y arasındaki atamaya benzer bir atama yaptık. Farklı olarak stack bölgesinde end_city isimli bir değişken oluşturuldu ve start_city'deki adres, uzunluk ve kapasite bilgileri buraya kopyalandı
                               // yani end_city'de start_city'nin heap'te referans ettiği veriyi işaret etmekte
    println!("{}", end_city); // Bu noktada start_city'nin ömrü dolar. Artık sadece end_city geçerlidir
                              // println!("City is {}", start_city); // Burada derleme zamanı hatası alınır.
                              /*
                                  start_city'yi end_city'ye almak scope dışına çıkıldığında bir hataya neden olur.
                                  drop fonksiyonu her iki değişken içinde çalışacağından Double Free hatası oluşur ve bellek güvenliği(memory safety) kaybolur.
                                  Bu nedenle Rust aslında start_city'nin stack'teki bilgilerini (adres, uzunluk, kapasite) end_city'ye alırken, start_city'yi de geçersiz kılar.
                                  Ancak yine de istersek heap bölgelerinin de birer kopyasını çıkartabiliriz. Deeply Copy
                              */
    let name = String::from("niklas");
    let copy_of_name = name.clone(); // deeply copy. Artık stack ve heap'te iki ayrı kopya var. Ancak bunun maliyeti yüksektir. Hem temizleme sırasındaki ek operasyon yüzünden hem de programın çalıştığı bellek alanının büyümesi nedeniyle
    println!("{} {}", name, copy_of_name);

    /*
        String gibi Heap kullananlar ile stack'i kullananların fonksiyonlara parametre olarak geçtikleri zamanki duruma bir bakalım.
        Sonrasında stack üzerinde duran ve dahili copy işlemine destek veren türlere(i32 mesela)
    */
    let words = String::from("blue,red,green,gold,pink");
    process_word(words); // burada move işlemi söz konusu yani artık words oyun dışı kaldı
                         //println!("{}", words); // burada derleme zamanı hatası alınır

    let my_lucky_number = 32;
    process_number(my_lucky_number); // my_luck_number, fonksiyona kopyalanarak geçti. Yani stack'teki konumu halen daha geçerli
    println!("{}", my_lucky_number); // bu nedenle my_lucky_number scope'taki konumunu korumaya devam ediyor

    /*
        O zaman soru geliyor.
        Örneğin bir String değişkeni bir metoda ille de referans olarak geçirmek istersem ne yapacağım?

        find_world_length metodundaki word, atama sonrası quote değişkeninin stack'teki adres alanını referans eden bir değere sahip olur.
        sadece adres bilgisini taşır, quote üstünde bir sahipliği yoktur.
    */
    let quote =
        String::from("Zaman su misali akıyor.Engel tanımadan, duraksamdan, geriye dönmeden");
    let l = find_word_length(&quote);
    println!("'{}' cümlesinin uzunluğu {} karekterdir", quote, l); // referans türünden taşıma nedeniyle quote hala oyunun içinde(scope dahilinde yani)

    /*
        Referanslı değişkenlerin mutable olarak kullanılmasında dikkat edilmesi gereken bir nokta var.
        Bir referansı mut kelimesi ile mutable yapabiliyoruz ancak aynı scope içinde sadece bir kere yapılabiliyor.
        Yani aşağıdaki kor parçası geçersiz.

        your_quote referansını aynı scope içinde mutable olarak iki değişkene almamız kısıtlanmıştır.
        Amaç çalışma zamanında birden fazla pointer'ın aynı bellek adresine erişmesine müsaade etmemektir.
        Data Races adı verilen bu durum uygulamanın çalışma zamanında beklenmedik davranışlar sergilemesine neden olur.
        Rust bunu henüz derleme aşamasında engellemek ister. O nedenle aşağıdaki kod build olmaz.
        Elbette farklı scope'lar kullanarak bu durum aşılabilir.

        Diğer yandan aynı scope'da bir mutable ve n sayıda immutable referansa izin verilmektedir
    */
    let mut your_quote = String::from("Hımm...");
    let s1 = &mut your_quote;
    let s2 = &mut your_quote;
    println!("{} {}", s1, s2);
}

fn process_word(word: String) {
    println!("{} üstünde işlemler...", word);
}

fn process_number(number: i32) {
    println!("{}", number);
}

// parametrenin referans olarak taşınması
// word & bildirimi ile bir sahiplik değil referans beklediğini söyler
// Rust dilinde fonksiyonların referans tipinden parametre almasına Borrowing deniliyor
fn find_word_length(word: &String) -> usize {
    // word.push_str(" - Anonim"); // borrowing durumlarında bu kullanıma izin verilmez. Derleme zamanı hatası alınır. Ancak bir istinsa var. word parametresi mutable hale getirilir. (word: &mut String) şeklinde
    word.len()
} // scope dışına çıktığımız yer. word bir sahiplik taşımadığı için metodun çağırıldığı yerdeki quote değişkeni oyunda kalmaya devam eder
