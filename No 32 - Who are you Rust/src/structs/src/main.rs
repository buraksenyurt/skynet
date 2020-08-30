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
