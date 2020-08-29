fn main() {
    /*
        IMMUTABLE / MUTABLE

        Rust dilinde değişkenler varsayılan olarak immutable karekteristiktedir.
    */
    let point = 90;
    println!("Sınav puanın {}", point);
    // point += 10; // Yandaki atamaya izin verilmez. Derleme zamanı hatası alınır.

    // Ancak immutable bir tipin mutable yapılarak sonradan değerinin değiştirilebilmesi de mümkündür.
    // fight değişkeni mutable olarak işaretlemiştir. Bu nedenle değeri değiştirilebilir.
    let mut fight = "Dı dıp dı dıp dı dıp dı dı dıp dı";
    println!("Mortal Combat {0}", fight);

    fight = "dı dı dıı dı dı dı dı dıı dıı dı dı dı dıııd";
    println!("Mortal Comat(Remix) {}", fight);

    /*
        CONSTANT

        const ile sabit değişkenler tamınlanabilir. Bunlar sadece tanımlandıkları gibi immutable değildir. Daima immutable'dır.
        Bir constant tipi tanımlanırken tür belirtilmelidir. İsimlendirme standardı da aşağıdaki gibidir
    */
    const ALWAYS_ON: bool = false;
    println!(
        "Always on mode is {}",
        match ALWAYS_ON {
            true => "Active",
            false => "Passive",
        } // Şu match ifadesinin kullanımını biraz daha anlayayım diye
    );

    /*
        SHADOWING

        let ile tanımladığımız immutable bir değişken(ki varsayılan olarak da öyle zaten)
        tekrardan let kullanılarak yeni bir değer alabilir ve hatta değişken türü de değişime uğrayabilir.
        Buna shadowing deniyor. İkinci let kullanımı ile birlikte ilk değişkenin değeri gölgede bırakılıyor.
        shadowing immutable tipler için geçerli bir durum.
    */
    let value = 23.93;
    let value = value + 0.58; // Burada shadowing söz konusu.
    println!("Value = {}", value); // En azından buradaki gibi value değişkenini kullanmazsak derleme zamanında Warning mesajı görürüz
    let value = true; // hatta burada shadowing olmakla kalmıyor veri türü de değişiyor
    println!("Value = {}", value);

    /*
        DATA TYPES

        Rust statik tür kullanan bir dildir. Dolayısıyla derleme noktasına gelindiğine her değişkenin türü bellidir.
        Veri tileri saysıla (Scaler) ve bileşik (Compound) olmak üzere ikiye ayrılır.

        Integer Tipi
        Bit     Signed  Unsigned
        8-bit	i8	    u8
        16-bit	i16	    u16
        32-bit	i32	    u32
        64-bit	i64	    u64
        128-bit	i128	u128
        arch	isize	usize İşlemcinin 32 bit veye 64 bit olma durumuna göre boyutlanır

        Floating Point Tipi
        f32
        f64

        Bunlar haricinde bool, char (4 byte'lık Unicode Scalar türüdür ve neredeyse her dilden karakteri destekler)

        COMPOUND(Bileşik Türler)
        Rust dilinde önceden tanımlı iki bileşik tür vardır. Tuple ve Array
        Tuple tipinde farklı türlerden değişkenleri bir liste olarak tutabiliriz.
        Array tipi ise sadece aynı türden değişkenleri barındırabilir.
        Array'lerde eleman sayısı sabittir. Veriyi stack üzerinde tutmak istediğimizde idealdir. Aksi durumda Vector tipi tercih edilebilir.
    */
    let pi = 3.14; // tip belirtmesekte Rust eşitliğe bakarak varsayılan bir tür ataması yapar
    let ageLimit: u8 = 12; // pek tabii veri türünü bilinçli olarak söyleyebiliriz de (u8 - 8 Bit Unsigned Integer oluyor)
    let limit: u8 = "18".parse().expect("Dönüştürme hatası"); // Tipler arası dönüşüm de söz konusudur. Bu durumda da dönüştürülecek veri türü söylenmelidir
    let eular: f32 = 2.76666666; // 32 bit floating point. Bir şey belirtmezsek f64 türünü alır

    let basket = ("Lamba", true, 1.90, 3.14, 10); // Burada basit bir tuple tanımı söz konusu.
    println!("{} {}", basket.0, basket.3); // tuple içindeki farklı yerlerdeki elemanlara bu şekilde erişebiliriz.
    let (a, b, c, _, e) = basket; // pattern matching ile tuple içeriğini bu şekilde değişkenlere de alabiliriz. Hatta _ ile atlayabiliriz de. (Bu arada bu atamaya destructuring deniliyor)
    println!("{},{},{},{}", a, b, c, e);

    let numbers = [1, 5, 2, 6, 9, 3, 8, 15, 37, 99]; // Basit bir dizi tanımı
    println!("{}", numbers[2]);
    let colors: [char; 3] = ['R', 'G', 'B']; // Diziyi tanımlarken veri türü ve eleman sayısı da verilebilir
    println!("{}", colors[2]);
    let columns = [1; 10]; // Bu da değişik bir kullanım. 10 tane 1 den oluşan bir dizi tanımladık
    println!("{}", columns[9]);

    // let column = columns[11]; //11 numaralı indis olmadığı için derleme hatası oluşur. Hatta VS Code IDE'sinde bile altı kırmızı olarak çizilir

    /*

        FUNCTIONS

        Değer döndüren fonksiyonlarda eğer en son noktada işlem sonucu alınıyorsa return kelimesini kullanmak zorunlu değildir,
        fonksiyonun farklı noktalarında dönüş vermek istersek return kullanabiliriz
    */
    println!("4+7={}", sum_of_two(4, 7));

    // fibonacci fonksiyonunu da çağıralım
    println!("13ncü sıradaki fibonacci sayısı {}", find_fibonacci(13));

    /*
        LOOPS

        Birkaç döngü örneği de koyalım. Rust dilin üç döndü çeşidi var loops,for ve while
    */
    // iter fonksiyonu ile yukarıdaki numbers dizisi elemanlarında ileri yönlü hareket edebiliriz
    for nmbr in numbers.iter() {
        println!(
            "{} {}",
            nmbr,
            if nmbr % 3 == 0 {
                // burada satır içi if koşulu kullandım
                "Tek sayı"
            } else {
                "Tek sayı değil"
            }
        );
    }

    // for döngüsünü ters yönlü olarak da kullanabiliriz
    // 1den 10a kadar tanımlı bir sayı aralığında geriye doğru gidiyoruz
    for nmbr in (0..11).rev() {
        print!("{}...", nmbr);
    }
    println!("On Live");

    // loop örneği. Koşula bağlı tekrarlı kod parçaları için tercih edilebilir

    // Sonsuz döngü örneği
    // loop {
    //     println!("Aghh!!! Çıkarın beni burdan");
    // }

    let mut counter = 0;

    let result = loop {
        // loop içerisinde break ile çıktığımızda döndürdüğümüz değer bu değişkene atanır
        counter += 1;
        if counter == 10 {
            break "işlemler bitti";
        }
    };
    println!("{}", result);
}

fn sum_of_two(x: i32, y: i32) -> i32 {
    x + y // return dememize ve ; koyduğumuzda hata alırız
}

// n'nci sıradaki fibonacci sayısını bulan fonksiyon
fn find_fibonacci(n: u32) -> u32 {
    match n {
        // pattern matching kullandık
        0 => 1,                                             // n sayısının 0 olma hali
        1 => 1,                                             // n sayısının 1 olma hali
        _ => find_fibonacci(n - 1) + find_fibonacci(n - 2), // n sayısının bunlar 0 ve 1 dışında olma hali
    }
}