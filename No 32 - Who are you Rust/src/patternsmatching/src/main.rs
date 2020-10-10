/*
    Patterns Matching kullanımları

    Pattern(şablon), basit veya karmaşık bir tipin yapısını eşleştirme yoluyla kontrol etmeye yarayan bir söz dizimi olarak düşünülebilir.
    pattern'leri Rust dilinde bir çok yerde kullanabiliriz. if, for, while ve diğerleri.
    Aşağıdaki örneklerde bu kullanım alanlarını anlamaya çalışıyorum.
*/

fn main() {
    /*
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
}

fn move_left(&(x, y): &(i32, i32), v: i32) -> (i32, i32) {
    (x + v, y + v)
}
