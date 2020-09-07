use std::fs::File;
use std::io;
use std::io::Read;

/*
    Hata yönetimi.

    Rust dili hataları iki kategoriye ayırıyor. Kurtarılabilir olanlar(recoverable) ve tabii ki kurtarılabilir olmayanlar(unrecoverable)
    Managed bir ortam olmadığını da biliyoruz. Dolayısıyla bir exception yönetim sistemi de bulunmuyor.

    Kurtarılabilir hatalarda kullanıcının uyarılması ve yeniden deneme yapılması mümkün.
    Kurtarılamayan hatalar ise tipik olarak çalışma zamanı bug'ları gibi düşünülüyor.

    Rust, kurtarılabilir hataların kontrolü için Result<T,E> tipini değerlendirmekte.
    Kurtarılamayan hatalar ise aslında ortamda bir panik havasının esmesi ve programın çalışmasının durması demek. Bu noktada,
    panic! makrosu ile karşılaşıyoruz. Hiç beklenmeyen ve geliştiricinin öngöremediği bir hata oluştuğunda çalışan panic! makrosu
    stack'i de temizleyip programın bir hata mesajı ile sonlanmasını sağlıyor.

    Winding: panic! makrosu çalıştığında rust ortamı çağırılan ne kadar fonksiyon varsa bunları takip ederek stack üzerinde bellek temizleme operasyonu icra eder.
    Tahmin edileceği üzere bu operasyon maliyetlidir. Eğer üretim ortamı dosyası hafifse winding devre dışı bırakılabilir ki buna Unwinding deniyor.

    Geliştirici olarak hangi durumda tekrar deneme yaptırılması yani hatadan dönülmeye çalışılması ve hangi durumda sürecin durdurulmasının kararını verebilmek gerekiyor.

*/
fn main() {
    // #1 Kendimiz de panik havası estirebiliriz
    // analyse_nickname(String::from("bam-bam"));
    // analyse_nickname(String::from("fck"));

    // #2
    // a_little_bit_panic(); // Yukarıdaki ikinci çağrım nedeniyle zaten bir panic oluştu ve program sonlandı. Dolayısıyla bu satır işletilmez

    // #3
    // Aşağıdaki çağrı Propagating Error senaryosu için örnektir.
    let r = load_file(String::from("./Crgo.toml")); //./Cargo.toml ile de deneyin. Yani var olan metinsel bir dosyanın da okunabiliyor olması lazım
    match r {
        Ok(content) => println!("{}", content),
        Err(e) => println!("Dosya okumada hata -> {}", e),
    }
    println!("\nYine biz işimize bakalım...\n");

    // Şimdi burada ? operatörünün kullanıldığı çok daha kısa kod bloğu içeren fonksiyonu kullandık
    let ct = load_file_content(String::from("nowhere.txt"));
    match ct {
        // Yine dönen içeriği ele aldık
        Ok(s) => println!("{}", s), //Hata yoksa dosya içini ekrana basıyor
        Err(e) => println!("Hata Bilgisi -> {}", e), // hata varsa error bilgisini yazdırıyoruz
    }

    println!("\nHatayı kontrol altında tutuyoruz\n");

    /*
        #4 Result<T,E> tipinin kullanışlı iki fonksiyonu vardır (unwrap ve except)
        unwrap, işlem başarılı ise Ok içinde ne dönmesi gerekiyorsa onu döner ve bir hata durumunda otomatik panic! makrosunu tetikletir.
        match deseni ile uğraşmaya gerek kalmaz.
    */
    let cargo_file = File::open("./Cargo.toml").unwrap(); // eğer dosya varsa File nesnesini döndürür.
    let unknown_file =
        File::open("olmayan.txt").expect("Bu dosya sistemde yok veya bozulmuş olabilir.");
    // panic! makrosu çalışması halinde burada yazdığımız mesaj trace içeriğine alınacaktır.
}

/*
    Fonksiyon, parametre olarak gelen dosyası açıp içeriğini geri döndürmek istemekte.
    Ancak sistemde olmayan bir dosya da söz konusu olabilir.
    Burada early return adı verilen hata kontrol senaryosu ele alınıyor. Yani bir hata oluştuğunda bunun bilgisi çağıran yere döndürülüyor.
    panic! çağrısı yerine hata mesajını object user'a veriyoruz.
*/
fn load_file(file_name: String) -> Result<String, io::Error> {
    let f = File::open(file_name); // dosyayı açmaya çalışıyoruz

    // Pattern Matching ile Result<T,E> sonucuna bakıyoruz.
    let mut f = match f {
        Ok(file) => file, // Dosya açılabildi, her şey yolunda. Aşağıda içeriğini okuyacağız
        Err(error) => return Err(error), // Error oluştu ve bunu fonksiyonu çağırdığımız yerde ele alabiliriz
    };

    let mut content = String::new();

    // şimdi dosya içeriğini okumaya çalışıyoruz ve yine hata olma durumunu ele alıyoruz
    match f.read_to_string(&mut content) {
        Ok(_) => Ok(content), // sorun yok ve Ok ile dosya içeriğini geriye dönüyoruz
        Err(error) => return Err(error), // sorun var geriye hata bilgisini verelim
    }
}

/*
    Yukarıdaki dosya okuma ve içeriğini döndürme fonksiyonunun çok daha kısa hali aşağıdaki gibi.
    Ama tabii burada olmayan veya içeriği okunamayacak dosyalar Error dönecektir
    ? operatörünün kullanımına dikkat.
*/
fn load_file_content(file_name: String) -> Result<String, io::Error> {
    let mut content = String::new();
    File::open(file_name)?.read_to_string(&mut content)?;
    Ok(content)
}

fn analyse_nickname(message: String) {
    if message == "fck" {
        panic!("Hey dostum, ne dediğinin farkında mısın?"); // Programı burada sonlandırıyoruz.
    } else {
        println!("Bana `{}` dedin", message);
    }
}

fn a_little_bit_panic() {
    let points = vec![0..10]; // burada bir vector dizisi oluşturduk
    println!("{:?}", points[11]); //ve burada da 11nci elemana ulaşmak istedik ki yok. Bu satırda panic! makrosu devreye girecektir
}
