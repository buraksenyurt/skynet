/*
    join() kullanımı ile ilgili başka bir kod parçası.
    Bu sefer n adet thread'i bir döngü ile başlatıyor
    bu döngülerde tamamen sembolik olarak uzun sürecek aynı işleri planlıyor
    ve main thread'i her birinin sonlanması için bekletiyoruz.
*/
use std::thread;
use std::time::{Duration, SystemTime}; // Zaman ölçümlemeleri için ekledik

fn main() {
    let now = SystemTime::now();

    /*
        Döngü başlatılan thread'leri bir vector'de topluyor.

        Eğer move closure'ını kullanmazsak i değişkeni sahipliğinin ödünç olarak thread içerisine alınamamasından dolayı
        derleme zamanı hatası alırız.
    */

    let mut threads = vec![];
    for i in 0..5 {
        threads.push(thread::spawn(move || {
            println!("{} başladı", i);

            for j in 1..5 {
                println!("Thread #{} da {} için bir şeyler yapılıyor gibi...", i, j);
                thread::sleep(Duration::from_secs(1));
            }

            return i; // thread'den geriye bir değer döndürüyoruz. Bu değeri aşağıdaki pattern matching kullanımında yakaladık

            // println!("{} sonlandı", i);
        }));
    }

    // Bitmeyen thread'ler için Main bekletiliyor.
    for t in threads {
        let result = t.join();
        match result {
            Ok(r) => {
                println!("#{} tamamlandı", r); // Tamamlanan thread'den dönen değeri r ile alabiliriz
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
    match now.elapsed() {
        Ok(elapsed) => {
            println!("Tüm işlemler için geçen toplam süre {}", elapsed.as_secs());
        }
        Err(e) => {
            println!("Hata oluştu: {:?}", e);
        }
    }
}
