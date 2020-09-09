/*
    Trait OOP tarafından gelen birisi için interface'lere benzetilebilir.
    Esasında struct'ların sahip olması istenen davranışları belirten metotların tanımlandığı bir sözleşmedir.
    Yani metotların neye benzeyeceğini tanımlar ortak bir deklarasyon sunarız.
    Diğer yandan trait'ler iş yapan gövdeleri olan fonksiyonlar da içerebilir. Bu açıdan biraz da abstract sınıflara benzettim.

    Rust standart kütüphanesi de birçok trait tanımı içerir. Add, Copy, Clone, Eq vb
*/

use std::ops; // + operatörünü tekrardan programlamak için eklendi (#4ncü örnek)

/*
    #1

    Action isimli bir trait.
    İçinde iki fonksiyon tanımı yer alıyor.

    Takip eden iki struct bu trait içerisinde fonksiyonları kendilerine göre uyarlıyorlar.
*/
trait Action {
    fn initialize(&self, x: i32, y: i32); // Trait fonksiyonları &self parametresine sahip olmalıdırlar. Elbette, başka parametreler de içerebilirler ve geriye döndürebilirler.
    fn click(&self) {
        println!("varsayılan bir click davranışı olsun diyelim"); // Varsayılan bir davranış icra ettik. Eğer click ezilirse(override) burası devreye girmez
    }
}

struct Button {
    name: String,
}
struct Hyperlink {
    url: String,
}

impl Action for Button {
    // Button struct'ı için Action trait'inin uygulanacağını söylüyoruz ancak sadece initialize metodunu ezdik.
    // Tabii click fonksiyonunun varsayılan bir kod bloğu olmasaydı onu da burada ezmek zorundaydık
    fn initialize(&self, x: i32, y: i32) {
        println!(
            "{} isimli düğme {}:{} noktasında oluşturuldu",
            &self.name, x, y
        );
    }
}

impl Action for Hyperlink {
    // Benzer şekilde Hyperlink struct'ı için de Action trait'inde belirtilen metotların uygulanacağının söylüyoruz
    fn initialize(&self, x: i32, y: i32) {
        println!("{} link kontrolü {}:{} noktasına eklendi", &self.url, x, y);
    }
    fn click(&self) {
        println!("Linke basılırsa {} adresine gidilir", &self.url);
    }
}

fn main() {
    let submit = Button {
        name: String::from("btnSubmit"),
    };
    let go_home = Hyperlink {
        url: String::from("https://www.buraksenyurt.com"),
    };
    submit.initialize(10, 20);
    submit.click();

    go_home.initialize(15, 30);
    go_home.click();

    /*
        #2

        Şimdi gelelim trait'lerin güzel kullanımlarından birine.
        Yukarıdaki kullanım çok anlam ifade etmiyor çünkü.
        Bu nedenle on_load fonksiyonuna odaklanalım. Parametre olarak Action trait'ini uygulayan tipleri kabul etmekte.
        Dolayısıyla Action trait'ini implement eden struct değişkenlerini aynı fonksiyonu içinde ele almamız mümkün.
    */
    on_load(&submit, 10, 20);
    on_load(&go_home, 20, 20);

    /*
        #3
        Tabii bunun üzerine akla, "e o zaman trait türünü kullanan vector tanımlayıp n adet struct için aynı operasyonu tetikleyelim" düşüncesi gelir
        Lakin trait'lerin boyutu yoktur ve bu nedenle bellekte ne kadar yer tutacakları bilinemez. Dolayısıyla düşündüğümüzü yapmak biraz beyin yakar.
    */

    println!("");

    let main_page = Hyperlink {
        url: String::from("azondot.com"),
    };
    let controls: Vec<Box<dyn Action>> = vec![
        Box::new(Button {
            name: String::from("help_me"),
        }),
        Box::new(main_page),
        Box::new(Button {
            name: String::from("next_page"),
        }),
    ]; // Box struct'ı heap'teki yer ayırımları için bir referans sunar.
    prepare(controls);

    /*
        #4 Operator Overloading

        C# taki gibi Rust dilinde de bilinen operatörleri yeniden programlayabiliriz.
        Örneğin kompleks sayıları temsil eden bir struct için + operatörünü yeniden programlamak istediğimizi düşünelim.
        + operatörünün karşılığı olan trait'i (Add) bu struct için yeniden programlamak yeterli olacaktır.
    */
    let cx1 = Complex { x: 1.23, y: 2.56 };
    let cx2 = Complex { x: 0.45, y: -4.89 };
    let cx3 = cx1 + cx2;
    println!("{} + ({})i", cx3.x, cx3.y);

    /*
        #5 Operator Overloading(drop)

        Bu arada değişkenlerin scope dışına çıktıları zaman devreye giren ve bellek boşaltma işini üstlenen drop'da bir trait'tir ve yeniden programlanabilir.
    */
    let london = MongoConnection {
        server: String::from("localhost"),
        port: String::from("3001"),
    };
    println!("{}:{}...", london.server, london.port); // london değişkenini kullandık ve scope dışında kaldı. Yazdığımız drop metodu devreye girecek
}

/*
    prepare fonksiyonu Action trait'ini uyarlayan yapılardan oluşan bir vector kabul eder.
    Bu sebeple Button ve Hyperlink nesnelerini içeren bir vector dizisini parametre olarak verip herbiri için aynı fonksiyonun çalıştırılmasını sağlayabiliriz.
    (Polymorphsym olabilir mi? Bir düşünelim)
*/
fn prepare(controls: Vec<Box<dyn Action>>) {
    let mut x = 5;
    let y = 10;
    for c in controls.iter() {
        // parametre ile gelen nesnelerin initialize fonksiyonu çalışır. Override edilmiş sürümleri
        c.initialize(x, y);
        x += 5;
    }
}

fn on_load<T: Action>(control: &T, x: i32, y: i32) {
    control.initialize(x, y);
}
/*
    Aşağıda on_load'un ilk versiyonu var.
    Yukarıdaki ise Trait Bound Syntax adı verilen sürümü. Bu versiyon tercih edilirse on_load'u çağırdığımız yerlerde Action değişkenleri için & kullanmamız gerekir.
*/
// fn on_load(control: impl Action, x: i32, y: i32) {
//     control.initialize(x, y);
// }

struct Complex {
    x: f32,
    y: f32,
}

// Complex struct'ı için Add operatörünü yeniden programlıyoruz
impl ops::Add for Complex {
    type Output = Self; // Kendi türünü döndüreceğini söylüyoruz ki bu Complex tip oluyor

    // add operasyonunu yeniden tanımlıyoruz
    fn add(self, c2: Complex) -> Self {
        Self {
            x: self.x + c2.x,
            y: self.y + c2.y,
        }
    }
}

/*
    #5 için kullanılan kobay struct ve drop uyarlaması.
    Mesela oluşturduğumuz MongoConnection nesnesi scope dışına çıktığında yapılmasını istediğimiz özel bir şeyler varsa,
    drop trait'inin yeniden programlayarak gerçekleştirebiliriz.
*/
struct MongoConnection {
    server: String,
    port: String,
}

impl Drop for MongoConnection {
    fn drop(&mut self) {
        println!(
            "{}:{} için belki bağlantı sonlandırma işini üstlenebiliriz.",
            self.server, self.port
        );
    }
}
