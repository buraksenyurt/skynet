/*
    thread'ler ile ilgili bu örnekte bir metin içerisindeki ç harflerinin sayısını buluyoruz.
    Ahış şahım bir örnek değil ama thread'ler için iyi bir antrenman.
    Senaryoya göre document değişkeni pipe işaretlerine göre parçalara ayrılıyor.
    Her bir parça üzerinde işlemler yapılması için birer thread açılıyor.
    Thread'ler içerisinde o thread'in ele aldığı içerikteki ç harfleri sayılıyor.
    Program sonunda bu değerler bir arada toplanarak ele alınıyor.
*/
use std::thread;

fn main() {
    // üzerinde çalışacağımız değişken
    let document = "Bugün epeyce Rust çalışmaya çalıştım|Çarşambanın gelişi bir önceki çarşambadan belli olur mu dersin|Kaç tane ç harfi yazalım|çççççççç demek istiyorum|Çok çalışmamız lazım...Çooookk çalışmamız";
    // thread'leri toplayacağımız vector
    let mut workers = vec![];
    // içeriği | işaretine göre ayrıştırdık
    let rows = document.split('|');

    for (i, row) in rows.enumerate() {
        println!("#{}->\"{}\"", i, row);

        // Burada yeni thread açıp workers'a ekliyoruz
        // Thread geriye kaç tane ç olduğunu dönecek
        workers.push(thread::spawn(move || -> u32 {
            let mut total = 0;

            for c in row.chars() {
                if c == 'ç' {
                    total += 1;
                }
            }
            println!("#{} içerisinde {} tane ç harfi var", i, total);
            total
        }));
    }

    let mut all_totals = vec![]; // işlenen her bir satır için bulunan toplam değerleri biriktireceğimiz vector(ne uzun cümle yazdım yahu)
    
    /*
        Tüm worker'ların işlerini bitirmesini bekliyoruz.
        Worker'ların işi bittikçe döndürülen sonuçlar(ç'lerin toplmaları)
        bir başka vector'de toplanıyor.
        En sonunda da genel toplamı yazdırıyoruz.
    */
    for worker in workers {
        let sub_total = worker.join().unwrap();
        all_totals.push(sub_total);
    }

    let sum = all_totals.iter().sum::<u32>();

    println!("Tüm dokümanda {} adet ç harfi varmış", sum);
}
