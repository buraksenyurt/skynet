/*
    Program komut satırından verilen dosyadaki satırları | ayracına göre ayırıp bir ürün koleksiyonunda toplamak için geliştirilmektedir.
*/

// Gerekli ortam kütüphaneleri
use std::env; // argümanları okurken
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process; // process'ten çıkartırken // Box trait'inden Error için

fn main() {
    let args: Vec<String> = env::args().collect(); // ekrandan girilen argümanları String türünden bir vector dizisine aldık

    /*
        unwrap_or_else fonksiyonu Non-Panic stilde çalışır.
        Aslında burada bir closure kullanımı da söz konusu.
        Dikkat edileceği üzere unwrap_or_else isimsiz bir fonksiyon çağırıyor ve bunu new'dan Err dönmesi halinde çalıştırıyor.
        Eğer new Ok dönerse kod akışı devam edecektir
    */
    let prmtr = Parameter::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1); // Uygulamadan çıkartır
    });

    println!(
        "`{}` dosya içeriği için `{}` işlemi yapılacak\n",
        prmtr.filename, prmtr.command
    );

    // ürün listesini çekiyoruz
    let products = read_product_lines(prmtr).unwrap_or_else(|e| {
        println!("Kritik hata: {}", e);
        process::exit(1);
    });

    for p in products {
        println!("{}", p); // Product struct'ına Display trait'ini implemente ettiğimiz için bu ifade geçerlidir.
    }
}

/*
    Terminalden gelen agrümanları Parameter isimli bir struct'ta toplayabiliriz.
    Ayrıca doldurulması için de bir constructor kullanabiliriz. (new metodu)
*/
struct Parameter {
    command: String,
    filename: String,
}

impl Parameter {
    // Constructor
    fn new(args: &[String]) -> Result<Parameter, &'static str> {
        /*
            Ekrandan girilen argüman sayısını kontrol edelim.
            Aslında iki parametre isterken 3 tane kontrol etmemiz tuhaf değil mi?
            Nitekim cargo kelimesinden sonra gelen run komutu da terminal argümanı sayılıyor.
            Yani run komutundan sonra gelen argümanları ele alacağız.
        */
        if args.len() != 3 {
            return Err("Argüman sayısı 2 olabilir"); // Panic yerine Error mesajı döner
        }
        let command = args[1].clone();
        let filename = args[2].clone();

        Ok(Parameter { command, filename }) // Sorun yoksa Parametre örneği döner
    }
}

/*
    read_lines fonksiyonu argümanların toplandığı Parameter struct'ını kullanır ve dosya içeriğini satır satır okur.
    Bu fonksiyonda non-panic stilde yazılmıştır.
    Geriye Ok veya hata durumuna göre Error trait'ini uygulayan hata referansları dönebilir.
    Ne tür bir hata döneceğini bilemediğimiz için dynamic trait kullanılmıştır.
    ?'te panic yerine Ok veya Error durumlarını döndürmektedir.
*/
fn read_product_lines(prmtr: Parameter) -> Result<Vec<Product>, Box<dyn Error>> {
    let file = File::open(prmtr.filename)?;
    let reader = BufReader::new(file);
    let mut products: Vec<Product> = Vec::new();

    // buffer'a gelen satırlarda ileri yönlü hareket ediyoruz
    for (_, line) in reader.lines().enumerate() {
        let row = line?;
        // println!("{}. {}", i + 1, row);
        // pipe işaretine göre satırı parse edip bir sütunları bir vector'e alıyoruz
        let columns: Vec<&str> = row.split("|").collect();

        // yeni bir Product değişkeni oluşturup alanlarını atıyoruz
        let prd = Product {
            id: columns[0].parse::<i32>().unwrap(),
            description: String::from(columns[1]),
            price: columns[2].parse::<f32>().unwrap(),
            quantity: columns[3].parse::<i32>().unwrap(),
        };

        // ve products isimli vector dizisine ekliyoruz
        products.push(prd);
    }

    Ok(products) // Buraya kadar sorunsuz geldiysek ürün listesini tutan vector'ü geriye dönüyoruz
}

struct Product {
    id: i32,
    description: String,
    price: f32,
    quantity: i32,
}
/*
    Display trait'ini Product struct'ımız için uyguluyoruz.
    Böylece println! makrosunda buradaki formatta ekrana bilgi yazdırılması mümkün.
*/
impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] - {}. Birim Fiyat {}. Stokta {} adet var.",
            self.id, self.description, self.price, self.quantity
        )
    }
}
