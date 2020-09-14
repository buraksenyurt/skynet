/*
    iterator deseni pek çok programlama dilinde yer alıyor.
    Amaç bir nesne dizisinde ileri yönlü hareket ederken her bir dizi öğesi için bir fonksiyonelliği çalıştırmak.
    Rust dilinde de böyle bir mekanizma mevcut.
    Acaba closures2'de yazmaya uğraştığım search fonksiyonu iterator trait'leri ile daha kolay olabilir mi :|

    iterator'lar tembeldir (lazy). Bunu iter() fonksiyonundan sonra başka bir tane çağırana kadar işlevsel değildir diye yorumlayabiliriz.
*/
fn main() {
    // en basit haliylet iterator kullanarak örneğin bir vector dizisi elemanları dolaşılabilir
    let average_points = vec![12.5, 20.9, 16.8, 7.9, 15.0];
    let iterator = average_points.iter(); // iterator tanımlandı
    for point in iterator {
        //burada da elemanlar ileryi yönlü dolaşılmaya başlandı
        println!("{}", point);
    }
}
