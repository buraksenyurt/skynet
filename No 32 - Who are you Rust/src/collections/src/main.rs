/*
    Her ne kadar Rust'ın built-in pek çok veri tipi stack bellek bölgesini kullanıyor olsa da
    koleksiyonlardaki gibi heap'de tutulan, dolayısıyla derleme zamanında ne kadar yer tutacağının
    bilinmesine gerek duyulmayan veri tipleri de mevcuttur.

    Koleksiyon türlerinin kabiliyetleri farklılık göstermekle birlikte duruma göre tercih edilirler.

    Rust dilinde en sık kullanılan koleksiyonlar belli türden değişkenlerden oluşan vector (minions'daki vector mü acaba :D ),
    karakter katarı koleksiyonu olan string ve key-value düzeninde çalışan hash map'tir.
*/

fn main() {
    /*
        vector tipi ile başlayalım.
        İlk satırdaki tanımlanma şeklinden de anlaşılacağı üzere vector generic bir koleksiyondur.
        Sadece <T> ile belirtilen türde elemanlar barındırır.

        Bir vector'ü new ile tanımlayabileceğimiz gibi macro ile de tanımlayabiliriz (! işareti olan fonksiyonlar)

        Tahmin edileceği üzere vector türü de varsayılan olarak immutable'dır.
        Bu nedenle colors isimli vector'e push metodu ile yeni elemanlar ekleyebilmek için,
        mut ile mutable olarak işaretlenmesi gerekmiştir.
    */
    let points: Vec<i32> = Vec::new(); // Şu anda i32 türünden elemanlar taşıyacak bir vector koleksiyonu tanımladık

    {
        // Elbette scope kanunları vector türü için de geçerlidir
        let mut colors = vec!["red", "green", "blue"]; // bu durumda vector'ün kullandığı tip sağ tarafa göre tahminlenir(infer)
        colors.push("purple"); //push sona eleman ekler
        colors.push("yellow");
        colors.push("pink");

        let last = colors.pop(); // pop ile son eklenen eleman elde edilir. aynı zamanda koleksiyondan da çıkartılır
        println!("{:?}", last);
    } // şu andan itibaren colors ve içeriğindeki tüm veriler bellekten atılmıştır (drop)

    // iterator dizileri kolayca bir vector'e alınabilirler
    let mut numbers: Vec<i32> = (10..20).collect();
    let x = numbers[5]; // vector içindeki herhangi bir elemana indis değeri üstünden erişebiliriz
    println!("{}\n", x);
    // iter fonksiyonundan yararlanarak vector elemanları kolayca dolaşılabilir
    // for n in numbers.iter() {
    for n in &numbers {
        // & operatörü ile vector referansını elde edip for ile ilerleyebiliriz
        print!("{},", n);
    }
    println!("\n");
    /*
        Eğer iterasyon sırasıdan koleksiyon elemanlarında değişiklik yapmak isterse iter_mut fonksiyonundan yararlanabiliriz
        Tabii aşağıdaki kodun çalışabilmesi için numbers isimli vector'ün değişikliğe izin vermesi de gerekir.
        Bu nedenle numbers mut ile mutable hale getirilmiştir
    */
    for n in numbers.iter_mut() {
        *n += 10; // vector'de o an referans edilen değeri değiştirmek için *(dereference) operatörünü kullanıyoruz
    }
    println!("{:?}", numbers);

    /*
        vector'leri pattern matching tadından aşağıdaki gibi değerlendirebiliriz.
        get ile 1nci indisi ele alıyoruz.
        get fonksiyonu Option<T> döndürdüğü için Some, None durumlarını ele alabiliriz.
    */
    match numbers.get(1) {
        Some(21) => println!("1 indisine denk gelen eleman {}", numbers[1]),
        None => println!("Hayır değil"),
        _ => println!("Diğerleri için bir şey yapmaya gerek yok"), // 21 olma ve olmama durumu haricinde diğer durumları da kontrol etmemiz beklenir. Buraya yazmazsak derleme zamanı hatası alırız.
    };

    /*
        vector türü tek tiple çalışacak şekilde tanımlanmıştır.
        Eğer farklı veri türlerinden bir nesne koleksiyonu olarak kullanmak istersek enum veri yapısını kullanabiliriz.
        Product enum veri yapısını bu amaçlar ele alabiliriz.

        Eğer çalışma zamanında vector'ün tutacağı veri türleri belli değilse enum yerine trait nesneleri kullanabiliriz.
    */
    let data_row = vec![
        Product::Id(1001),
        Product::Title(String::from("12li Su Bardağı")),
        Product::ListPrice(12.90),
    ];
}

enum Product {
    Id(i32),
    Title(String),
    ListPrice(f32),
}
