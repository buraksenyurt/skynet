/*
    fearless_concurrency örneğinde ana thread işi bitirdiği için
    başlatılan ve halen devam eden diğer thread'lerinde sonlandığını gördük.
    Bunu önlemek için JoinHandle<T> tipinden yararlanabiliriz.
    Aşağıdaki örnek kod bu amaçla geliştirilmiştir.
*/
use std::thread;
use std::time::{Duration, SystemTime}; // Zaman ölçümlemeleri için ekledik

fn main() {
    let now = SystemTime::now();

    // spawn geriye JoinHandle<T> nesnesi döndürür
    let wait_handle = std::thread::spawn(|| {
        println!("1 başladı...");
        for _i in 1..7 {
            // Ekrana 10 defa Black yazacak
            println!("BLACK");
            thread::sleep(Duration::from_secs(2));
        }
        println!("1 bitti...");
    });

    println!("Ana thread başladı...");
    for _i in 1..5 {
        println!("White");
        thread::sleep(Duration::from_secs(1));
    }
    println!("Ana thread bitti...");

    // Main Thread'e ilk thread'in tamamlanmasını beklemesini söylüyoruz
    wait_handle.join().unwrap();

    match now.elapsed() {
        Ok(elapsed) => {
            println!("Tüm işlemler için geçen toplam süre {}", elapsed.as_secs());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
