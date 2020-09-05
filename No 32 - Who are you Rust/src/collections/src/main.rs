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

    /*
        Gelelim Rust standart kütüphanesi ile birlikte gelen diğer bir koleksiyon olan String'e.
        String'i aslında byte'lar koleksiyonu olarak düşünmek daha doğru olabilir.

        String'in birkaç oluşturulma şekli var. Örneğin new ile tanımlanıp literal bir string üstünden to_string çağrısı ile
        ya da from fonksiyonu ile üretilebililir.

        String veri türü UTF-8 formatında kodlanmış içerikleri kullanabilir. Bu sebepten whatsup değişkeninde olduğu gibi pek çok dili destekler.

        String'leri birleştirmek veya bir String'e başka bir String parçası eklemek için push_str ve
        tek bir karakter eklemek için push fonksiyonlarını kullanabiliriz.

        Tabii + operatörü de String'leri birleştirmek için kullanılabilir.
        Çok fazla birleştirilecek String varsa + operatörü (ki add fonksiyonuna karşılık gelir) yerine,
        format! isimli macro'yu kullanmak daha uygundur.

    */
    let mut aloha = String::new(); // aşağıda değerini değiştireceğimiz için mutable yaptık
    let incoming_data = "Alofortanfane";
    aloha = incoming_data.to_string();

    println!("{}", aloha);

    let raining_day = String::from("Một ngày mưa.");
    println!("{}", raining_day);

    let mut quote = String::from("Siz ne kadar veri üretirseniz");
    quote.push_str(", organize suç örgütleri de o kadar tüketir");
    quote.push('!');
    println!("{}", quote);
    quote.push_str(" Marc Goodman - Geleceğin Suçları");
    println!("{}", quote);

    /*
        + operatörünü kullandığımızda & ile referans adreslerine ulaşmamız gerekir.
        Bunun sebebi aslında + operatörünün işaret ettiği add metodunun (fn add(self, s: &str) -> String şeklinde yazılmıştır)
        &str şeklinde referans istemesidir.
    */
    let s1 = "Ne".to_string();
    let s2 = String::from("güzel");
    let s3 = String::from("bir");
    let s4 = String::from("gün!");
    let last_word = s1 + " " + &s2 + " " + &s3 + " " + &s4; //s1'e sırasıyla s2, s3 ve s4 değişkenlerinin referans adresleri eklendi
    println!("{}", last_word);

    let black = String::from("black");
    let white = String::from("white");
    let black_and_white = format!("{} {} {}", black, "or", white);
    println!("{}", black_and_white);

    /*
        String veri türünde uzunluk aslında kullanılan karakterlerin byte olarak kapladıkları yere göre değişir.
        Eğer Unicode karakter varsa bu UTF-8 kodlaması sonucu 2 byte olarak ele alınır ve uzunluk değişir.
        Belki de bu sebepten ötürü String türünde indis operatörü kullanılamaz.
    */
    let siyah = "đen";
    println!(
        "Siyah vietnamca `{}` olarak yazılır. Rust için uzunluğu {}. Halbu ki sen 3 karakter saydın :)",
        siyah,
        siyah.len()
    );
    // let second = siyah[1]; // the type `str` cannot be indexed by `{integer}` hatası döner

    /*
        Bir String içinden belli bir dilimi almak (slice) mümkündür ancak dikkat etmek gerekir.
        Çünkü denk gelen byte bir karakter olarak ifade edilemeyebilir.
        Aşağıdaki kod parçası derlenecektir ama çalışma zamanında bir panic oluşacaktır.
        thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'đ' (bytes 0..2) of `đen`'
    */
    // let a_bit_off_word = &siyah[0..1];
    // println!("{}", a_bit_off_word);

    /*
        String içerisindeki karakterleri veya byte'ları dolaşmanın en güzel yolu chars ve bytes fonksiyonlarından yararlanmaktır
    */
    println!();

    let rusca_bir_seyler = String::from("Добрый день умереть.");

    for c in rusca_bir_seyler.chars() {
        print!("{} ", c);
    }
    println!();
    for b in rusca_bir_seyler.bytes() {
        print!("{} ", b);
    }
    println!();

    /*
        String ile başka neler yapabiliriz bakalım.
        Mesela String içindeki karakterleri bir vector'e indirebiliriz.
    */
    let char_vector: Vec<char> = rusca_bir_seyler.chars().collect();
    for c in char_vector {
        println!("`{}` ", c);
    }
}

enum Product {
    Id(i32),
    Title(String),
    ListPrice(f32),
}
