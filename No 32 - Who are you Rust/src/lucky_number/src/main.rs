use rand::Rng; // rastgele sayı kütüphanesi
use std::cmp::Ordering; // Kıyaslama operasyonu için eklenen Enum. cmp(Compare) için match kullanılan yere dikkat
use std::io; // Standart kütüphane. Ekrandan girdi okumakta işimize yarayacak

/*
    Örnek kod parçasında sayı tahmin oyunu icra ediliyor.
    Yarışmacının 5 tahmin hakkı var.
*/

fn main() {
    println!("Sayı tahmin oyununa hoşgeldin...");
    println!("10 ile 30 arası bir sayı tuttum.\nBakalım bilebilecek misin?");

    // 1 ile 50 arası sayı üretiyor
    // thread_rng, rastgele sayı üretici nesnesini verir
    // get-range metodu ile
    let computer_number = rand::thread_rng().gen_range(10, 30);
    let mut player_guess: u8; // yine mutable bir pozitif tam sayı tanımı. 8 bitlik.

    println!("Hadi bir tahminde bulun");

    // 5 iterasyonlu bir döngü kurduk
    for i in 1..6 {
        let mut screen_input = String::new(); // Değeri değiştirilebilir bir String değişken (Mutable)

        println!("{}. hak", i);

        // Ekran girilen veriyi screen_input değişkenine alıyoruz
        io::stdin()
            .read_line(&mut screen_input)
            .expect("Okuma sırasında hata"); // Olası hata durumu mesajımız

        /*
            String değeri u8 tipine dönüştürüyoruz ama nasıl? :)

            parse metodu bir sonuç döner. Bu sonuç Ok veya Err değerleri içeren bir Enum sabitidir.
            Bu sabiti match ederek ne yapılacağına karar veriyoruz.
            Ok ise sorun yok. Yani dönüştürme başarılı olmuş.
            Lakin dönüştürme başarısızsa parse dönüşü Err demektir. Bu durumda ekrana mesaj yazdırıp continue ile döngüyü devam ettiriyoruz.
        */
        player_guess = match screen_input
            .trim() // neden trim'ledik
            .parse()
        {
            Ok(n) => n,
            Err(_) => {
                println!("Girdiğin sayıyı dönüştüremedim. Lütfen tekrar dene.");
                continue;
            }
        };

        /*
            cmp çağrısının sonucu Ordering sabitinin hangi durumu oluşuyorsa,
            ona göre bir kod parçası işletiliyor.

            match Arms Aşağıdaki şekilde bir kullanım söz konusu.

            match value {
                pattern => expression,
                pattern => { expressions }, // blokta olabilir
                pattern => expression,
            }
        */
        match player_guess.cmp(&computer_number) {
            Ordering::Less => println!("Tahminini yükselt"),
            Ordering::Equal => {
                // Doğru tahmin etmişse döngüden çıkartırız
                println!("Bingo!!!");
                break;
            }
            Ordering::Greater => println!("Tahminini küçült."),
        }
    }

    println!(
        "Oyun tamamlandı ve benim tuttuğum sayı {} idi",
        computer_number
    );
}
