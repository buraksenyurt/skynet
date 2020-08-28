/*
    isimlendirme standardı olarak snake_case kullanılıyor.
    Mesela input_x yerine inputX kullanınca cargo check sonrası uyarılar alıyoruz.

    Bu ilk kod parçası ekrandan bir sayı değeri alıp faktöryelini buluyor.
*/

use std::io; // IO modülünü kullanacağımızı belirttik. stdin fonksiyonunu kullanabilmek için gerekli

fn main() {
    println!("Selam! Ben bir hesap makinesiyim :P");
    println!("X değeri?");

    /*
        Aşağıdaki iki değişken tanımı söz konusu.
        Rust dilinde değişkenler varsayılan olarak immutable'dır. Yani atamadan sonra değerleri değiştirilmez.
    */
    let mut input_x = String::new(); // bunun mutable olması gerektiğinden mut keyword'ü kullanıldı.
    let x: u32;

    io::stdin().read_line(&mut input_x).expect("Bir hata oldu"); // ekrandan girilen bilgiyi input_x'e okuyoruz (& sanıyorum pointer. İleride netleştirelim)

    x = input_x
        .trim()
        .parse::<u32>()
        .expect("Dönüştürme işleminde hata"); // ekrandan alınan bilgi 32bit integer'a dönüştürüyoruz

    let y = calculate(x); // hesaplama fonksiyonunu çağırıyoruz
    println!("x! = {}", y); // Sonucu ekrana basıyoruz

    // x = 9; // Değişkenler varsayılan olarak immutable olduğundan burada derleme hatası oluşur. x'e ikinci kez değer atayamayız.
}

/*
    Recursive çalışan fonksiyonumuz.
    Unsigned Integer 32 alıp aynı tipten sonuç dönüyor.
*/
fn calculate(num: u32) -> u32 {
    match num {
        // Pattern matching uygulanıyor
        0 | 1 => 1,                    // 0 veya 1 ise 1 döner
        _ => calculate(num - 1) * num, // bunlardan farklı ise sayıyı bir azaltıp yine kendisini çağırır
    }
}
