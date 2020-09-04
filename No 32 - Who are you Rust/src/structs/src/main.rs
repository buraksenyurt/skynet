/*
    OOP'taki gibi bir varlığı ve niteliklerini tanımlamanın yolu struct veri tipidir
*/

fn main() {
    // Product tipinde bir struct nesnesi örnekledik
    // Aksi belirtilmedikçe struct türleri de immutable'dır
    // Sonradan içeriklerinde değişiklik yapacaksak mut ile mutable hale getirilmelidir
    let mouse = Product {
        title: String::from("El Ci Kablosuz Mouse"),
        company: String::from("Azon Manufacturing Company"),
        unit_price: 44.50,
        stock_level: 100,
        is_usable: false,
    };
    write_to_console(mouse); //Ekrana bilgilerini yazıracağımı bir metot kullanayım dedim

    // println!("{}", mouse.title);
    // mouse.company = String::from("New Company"); // mouse değişkeni mutable tanımlanmadığı için mümkün değildir lakin mutable olsa da kod hata verecektir

    let monitor = create_product(
        String::from("Filips 24 inch monitor"),
        String::from("Northwind Enterteintmant"),
        340.50,
        45,
    );

    // Bir struct'ı diğer bir struct içeriğinden yararlanarak oluşturmak da mümkün (struct update sytnax)
    let monitor2 = Product {
        title: String::from("Soni viewsonic monitor"),
        ..monitor // Dikkat! Bu noktada monitor oyun dışı kalıyor(scope dışında). Neden?
    };

    write_to_console(monitor2);

    // Burada da tuple struct kullanımı söz konusu
    let persival = Player(String::from("Ready"), String::from("Player One"), 95);
    println!("{} {} {}", persival.0, persival.1, persival.2);

    /*
        Bir struct için tanımlanan metot kullanım örneği.
        struct yapısından değişkenler tanımladıktan sonra o değişken kapsamına dahil olan ilgili metotları çağırabiliriz.
    */
    let gudyonsen = Gamer {
        play_count: 17,
        penalty_point: 12,
        ability_rate: 3,
    };
    println!("{}", gudyonsen.get_level());
    println!("{}",gudyonsen.calc_reward());
}

struct Who {} // Yandaki gibi hiçbir alan içermeyen türden strcut ta tanımlanabiliyor. Trait konusunda önem kazanıyormuş. Henüz amacını anlayamadım
              /*
                  Birde tuple struct diye bir mevzu var.
                  Alan adları(field names) yok dikkat edileceği üzere.
                  Bu nedenle alan adlarına 0,1,2 gibi isimler üzerinden erişebiliyoruz.
              */
struct Player(String, String, i16);

// Parametrelerden yararlanarak geriye Product örneği döndüren fonksiyonumuz
fn create_product(title: String, company: String, unit_price: f32, stock_level: i16) -> Product {
    /*
        metot parametre adları ile struct alan adları aynı olduğu için aşağıdaki gibi bir kullanım mümkün.
        yani title:title, company:company gibi atamalar yapmak zorunda değiliz
    */
    Product {
        title,
        company,
        unit_price,
        stock_level,
        is_usable: false,
    }
}

fn write_to_console(p: Product) {
    println!(
        "\n{} ({})\n{} dalır.\nStokta {} adet var.\nŞu an satışta mı? {}",
        p.title,
        p.company,
        p.unit_price,
        p.stock_level,
        if p.is_usable { "evet" } else { "hayır" }
    );
}

// Product isimli bir struct
struct Product {
    title: String,
    company: String,
    unit_price: f32,
    stock_level: i16,
    is_usable: bool,
}

/*
    Struct veri yapısı için metotlarda tanımlanabilir.
    Ancak tanımlanma şekilleri fonksiyonlardan biraz farklıdır.
    Struct metotları, struct'ın kendi kapsamı içinde tanımlanır.
    Aşağıda Gamer struct'ı için iki metodun nasıl tanımlandığı gösterilmekte.
*/
struct Gamer {
    play_count: i16,
    ability_rate: i16,
    penalty_point: i16,
}

impl Gamer {
    fn get_level(&self) -> i16 {
        // self ile metodu imlpemente ettiğimiz veri yapısının çalışma zamanındaki örneğini işaret ederiz ki struct metotları &self referansı ile başlamak zorundadır
        return ((self.play_count * 10) - self.penalty_point) + self.ability_rate;
        // çalışma zamanındaki değişken değerlerine erişmek için de self. notasyonu üstünden ilerleriz.
    }
    fn calc_reward(&self) -> String {
        return String::from("Müthiş bir ödül kazandın");
    }
}
