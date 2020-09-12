/*
    Program komut satırından verilen dosyadaki satırları | ayracına göre ayırıp bir ürün koleksiyonunda toplamaktadır.
*/

// Gerekli ortam kütüphaneleri
use std::env; // argümanları okurken
use std::error::Error;
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

    if let Err(e) = read_lines(prmtr) {
        println!("Kritik hata: {}", e);
        process::exit(1);
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
fn read_lines(prmtr: Parameter) -> Result<(), Box<dyn Error>> {
    let file = File::open(prmtr.filename)?;
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        let row = line?;
        println!("{}. {}", i + 1, row);
    }

    Ok(())
}
