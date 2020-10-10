/*
    Patterns Matching kullanımları

    Pattern(şablon), basit veya karmaşık bir tipin yapısını eşleştirme yoluyla kontrol etmeye yarayan bir söz dizimi olarak düşünülebilir.
    pattern'leri Rust dilinde bir çok yerde kullanabiliriz. if, for, while ve diğerleri.
    Aşağıdaki örneklerde bu kullanım alanlarını anlamaya çalışıyorum.
*/

fn main() {
    /*
        #1
        Önce pattern(şablon) konusuna bir bakalım.

        İlginç geldi ki aşağıdaki ifadelerde soldaki değişkenler birer pattern'dir.
        Sağ taraftan ne gelirse gelsin eşleştirdiğimiz birer aktördür.

        let PATTERN = EXPRESSION;
    */
    let pi = 3.1415; // pi bir pattern
    let (x, y, z) = (1, 3, 5); // Burada eşitliğin sağındaki tuple verisini bir pattern ile eşleştirdik(match)
    let (a, b, _, d) = (1, 1, 0, 1); // _ ile pattern içerisindeki bir eşleşmeyi atladığımızı ifade ettik

    /*
        Aşağıdaki while let döngüsünde colors isimli vector'ün elemanlarını dolaşırken
        pattern matching kullanılmaktadır.
        pop metodu eleman yoksa None döner, eleman varsa da elemanı döner :)
        while let ile bu eşleşme Some(color) ile kontrol edilir.
        Böylece vector'den eleman çektikçe None veya herhangi biri olma eşlemesine(match) bakılır.
        Bu arada pop fonksiyonu hep son eklenen elemanı vereceğinden döngü renkleri ters sırada ekrana basar.
    */

    let mut colors = Vec::new();
    colors.push("Red");
    colors.push("Green");
    colors.push("Blue");
    while let Some(color) = colors.pop() {
        println!("{}", color);
    }

    /*
        Şimdi de aşağıdaki for kullanımına bakalım.
        Burada da (x,v) aslında bir pattern olarak karşımıza çıkar.
        enumerate fonksiyonu geriye iterasyondaki elemanın sıra numarası(index)
        ve değerini(value) döndürür.
        for (x,v) bu eşleşmeye bakar.
    */

    let market_list = vec![
        "Bir kilo prinç",
        "2 ekmek",
        "Yarım kilo un",
        "Bir paket dilimlenmiş kaşar peyniri",
        "Aç bitir salam",
    ]; // Evet bu kısımlarda acıkmışım

    for (x, v) in market_list.iter().enumerate() {
        println!("{} -> {}", x, v);
    }

    /*
        if let ifadelerinde de pattern matchin kullanılabilir.
        Aşağıdaki point değişkenin değeri String bir içerikten parse edilerek alınıyor.
        parse geriye Result<Value,Error> döner. Bu Ok() ile eşleştirilebilir.
        parse işlemi başarılıysa Result'ın Value değeri Ok(p) gibi döner.
        parse başarılı değilse Ok(p) eşleşmesi ihlal edilir ve else bloğuna girilir.
    */
    let point: Result<f32, _> = "3.1415".parse(); // Bide, float olarak Parse edilemeyecek bir şey yazıp deneyin
    if let Ok(p) = point {
        if p > 2.777 {
            println!("Harika bir iş");
        } else {
            println!("Belki biraz daha çalışmak lazım")
        }
    } else {
        println!("Problem var");
    }

    /*
        Fonksiyon parametreleri de birer şablon olabilir.
        Aşağıdaki örneğe bakalım.
        move_left fonksiyonuna gönderilen location isimli tuple, parametre tarafındaki &(x,y) şablonu ile eşleştirilir.
    */
    let location = (10, 20); //location bir pattern
    let (a, b) = move_left(&location, 5); // (a,b) de bir pattern
    println!("({}:{}) -> ({}:{})", location.0, location.1, a, b);

    // Aşağıdaki kod parçasını açınca bir uyarı mesajı alınır. Sizce neden?
    // if let value = 10 {
    //     println!("{}", value);
    // }

    /*
        Şablonları struct veri türünün değişkenlerini başka değişkenlere kolayca almak için de kullanabiliriz.
        Buna Destructing demişler. Belki de parçalarını çekip çıkardığımız içindir.
        Her neyse. Aşağıdaki kullanıma bakalım.
        Bu kod parçasında bird içindeki id ve nick_name bilgilerini let pattern ile sol taraftaki number, player_name isimli
        değişkenlere aldık(aynı isimli değişkenler de kullanabiliriz bu durumda : ile isim belirtmemize gerek yok)
        ve bir sonraki satırda kullanabildik.
    */
    let bird = Player {
        id: 33,
        nick_name: String::from("Leri Böörd"),
    };
    let Player {
        id: number,
        nick_name: player_name,
    } = bird;
    println!(
        "{} numaralı formasıyla '{}' geliyorrrr...",
        number, player_name
    );

    /*
        #2

        Biraz da match kullanımlarına bakıp hatırlayalım.
        match kullanımının belki de en basit hali aşağıdaki gibidir.
        currency şablonunun match ifadesindeki durumlardan birisine uygunluğu kontrol edilir.
        _ ile hiçbirisine uymayan durum söz konusudur
    */
    let currency = "TL";
    match currency {
        "TL" => println!("TL işlemi uygulanacak"),
        "USA" => println!("Dolar işlemi uygulanacak"),
        _ => println!("Hiç birisi uymadı"),
    }

    /*
        Şimdi aşağıdaki match kullanımına odaklanalım.
        value_a'nın sahip olduğu değeri kontrol ediyoruz.
        Some(50) eşleşmesi çalışmayacak, çünkü value_a Some(100) değerine sahip.
        Lakin ikinci Some kontrolü eşleşecek. Buradaki mutable value_b, value_a nın başlangıçtaki değerine sahip olur. Yani Some(100)'e.
        Bu nedenle eşleşme kabul edilir ve blok içerisinde value_b değeri 1 artırılıp ekrana basılır ki bu değer 101 dir!!!
        match sonrasında ekrana A ve B değerleri 100 ve 10 olarak basılacaktır.
        Burada kafalar karışabilir. Some(mut value_b) eşleşmesi çalışmıştı ve orada value_b değerini 1 artırmıştık. Dolayısıyla value_b'nin 101 kalması gerekir diyebiliriz.
        Ancak Some(mut value_b) value_a nın match ifadesinde kullanılmaktadır. Yani kendi bloğu içinde yeni bir değişkendir. Sadece value_a'nın başlangıç değerini almaktadır.
        Bunu etraflıca düşünüp hazmetmeye çalışın :)
    */
    let value_a = Some(100);
    let value_b = 10;

    match value_a {
        Some(50) => println!("Got 50"),
        Some(mut value_b) => {
            value_b = value_b + 1;
            println!("{}", value_b);
        }
        _ => println!("Farklı bir koşul"),
    }

    println!("A değeri {:?}, B değeri = {:?}", value_a, value_b);

    /*
        Aşağıdaki match ifadesinde, şablonların veyalanarak ve bir aralık
        belirtilerek kullanılması örneklenmektedir.
        Veyalamak için | aralık belirtmek içinse ..=(Matching Range) operatörlerinden yararlanılır.
        Matching Range sayı ve karakter veri tipi için kullanılabilir.
    */
    let order_no = 10;
    match order_no {
        1 | 2 | 3 => println!("İlk üçtesiniz. Sıranız * 3 puan verilir."),
        4 | 5 | 6 => println!("Yine de 1 puan verilir"),
        7..=10 => println!("7nci ve 10ncu arasındasınız. O zaman 0.5 puan verelim."),
        _ => println!("Kontenjan dışı kaldınız :("),
    }

    let first_letter = 'l';
    match first_letter {
        'a'..='m' => println!("{} izin verilen listede", first_letter),
        _ => println!("{} izin verilen listede değil", first_letter),
    }
}

fn move_left(&(x, y): &(i32, i32), v: i32) -> (i32, i32) {
    (x + v, y + v)
}

// Destrcuting örnekleri için
struct Player {
    id: i32,
    nick_name: String,
}
