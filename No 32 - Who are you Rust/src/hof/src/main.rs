/*
    HOF -> Yani Higher Order Functions

    Rust fonksiyon dil özelliklerini taşır demiştik. Bunlardan birisi de Higher Order Functions kabiliyetidir.
    Yani fonksiyonları birbirlerine bağlayıp yeni fonksiyonellikleri çalıştırabiliriz.
    Aslında benim aklıma hep . sonrası birbirlerine bağlanan LINQ metot zincirleri geliyor.

    Aşağıdaki örneği göz önüne alalım.
    500 ile 1000 arasındaki sayılardan, karesi 3 ve 5 ile bölünebilenlerin toplamını bulmak istiyoruz.
    İlk olarak bunu imperative yaklaşımla yapıyoruz.
    Sonrasında ise fonksiyonel dil yetenekleri ile.
*/
fn main() {
    let mut total = 0;

    // imperative yaklaşım
    for n in 500..1000 {
        let s = n * n;
        if calc(s) {
            total += s;
        }
    }
    println!("Imperative stilde toplam {}", total);

    // fonksiyonel yaklaşım
    let total2: i32 = (500..1000)
        .map(|n| n * n) // Aralıktaki sayıların karelerinden oluşan kümeyi bir alalım
        .filter(|&s| calc(s)) // bunların 3 veya 5 ile bölünebilme hallerine bakalım
        .fold(0, |t, s| t + s); // o kurala uyanları da toplayalım

    println!("Fonksiyonel stilde toplam {}", total2);
}

fn calc(n: i32) -> bool {
    n % 3 == 0 && n % 5 == 0
}
