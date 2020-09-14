/*
    iterator deseni pek çok programlama dilinde yer alıyor.
    Amaç bir nesne dizisinde ileri yönlü hareket ederken her bir dizi öğesi için bir fonksiyonelliği çalıştırmak.
    Rust dilinde de böyle bir mekanizma mevcut.
    Acaba closures2'de yazmaya uğraştığım search fonksiyonu iterator trait'leri ile daha kolay olabilir mi :|

    iterator'lar tembeldir (lazy). Bunu iter() fonksiyonundan sonra başka bir tane çağırana kadar işlevsel değildir diye yorumlayabiliriz.
*/
fn main() {
    // en basit haliylet iterator kullanarak örneğin bir vector dizisi elemanları dolaşılabilir
    let average_points = vec![12.5, 20.9, 16.8, 7.9, 15.0];
    let iterator = average_points.iter(); // iterator tanımlandı
    for point in iterator {
        //burada da elemanlar ileryi yönlü dolaşılmaya başlandı
        println!("{}", point);
    }

    /*
        iterator'lar standart kütüphanedeki Iterator isimli trait'i implemente ederler.
        Bunun içerisindeki next fonksiyonunu uygularlar. next ile hep bir sonraki elemana gidilir ve hatta sona gelip gelinmediği de anlaşılır.

        (Belki de kendi iterator'umuzu tanımlarken de bu trait'i uyarlamak yeterli olacaktır. İlerde göreceğim)
    */
    let mut iterator2 = average_points.iter(); // next ile hareket ederken iterator'un mutable olması gerekir
    println!("1. {}", iterator2.next().unwrap());
    println!("2. {}", iterator2.next().unwrap());
    println!("3. {}", iterator2.next().unwrap());
    println!("4. {}", iterator2.next().unwrap());
    println!("5. {}", iterator2.next().unwrap());
    //println!("6. {}", iterator2.next().unwrap()); // Upsss! Burada artık olmayan bir elemanı almaya çalışıyor. Panic! oluşur.

    /*
        iter() arkasından kullanılabilecek farklı fonksiyonlar da vardır.
    */
    let iterator3 = average_points.iter();
    let sum_of_points: f32 = iterator3.sum(); // bu noktanda sonra iterator3'ü yeniden kullanamayız. sum onun sahipliğini aldığı için.
    println!("Total points {}", sum_of_points);
    //let avg_of_points: f32 = iterator3.average();

    /*
        iter().map() fonksiyonu da oldukça kullanışlıdır.
        map fonksiyonu closure kullanır. Dolayısıyla iterasyon sırasında her eleman için çalışacak isimsiz fonksiyonlar kullanabiliriz.
        Aşağıdaki örnekte şehir adlarından toplam karakter sayısı 5in altında olanlar map ile tespit edilip ekrana yazdırılıyor.
    */
    let cities = vec![
        "istanbul", "izmir", "ankara", "new york", "chicago", "boston", "london", "tokyo", "van",
        "rize", "lizbon", "denver", "dublin",
    ];
    let founded: Vec<_> = cities
        .iter()
        .map(|name| {
            if name.len() < 5 {
                println!("{}", name);
                Ok(name)
            } else {
                Err(())
            }
        })
        .collect();
}