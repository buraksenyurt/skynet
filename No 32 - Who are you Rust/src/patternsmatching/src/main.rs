/*
    Patterns Matching kullanımları

    Pattern, basit veya karmaşık bir tipin yapısını eşleştirme yoluyla kontrol etmeye yarayan bir söz dizimi olarak düşünülebilir.
*/

fn main() {
    /*
        Aşağıdaki while let döngüsünde colors isimli vector'ün elemanlarını dolaşırken
        pattern matching kullanılmaktadır.
        pop metodu eleman yoksa None döner, eleman varsa da elemanı döner :)
        while let ile bu eşleşme Some(color) ile kontrol edilir.
        Böylece vector'den eleman çektikçe None veya herhangi biri olma eşlemesine(match) bakılır.
        Bu arada pop fonksiyonu hep son eklenen elemanı vereceğinden döngü renkleri ters sırada ekrana basar.
    */

    let mut colors = Vec::new();
    colors.push("Red");
    colors.push("Green");
    colors.push("Blue");
    while let Some(color) = colors.pop() {
        println!("{}", color);
    }

    /*
        Şimdi de aşağıdaki for kullanımına bakalım.
        Burada da (x,v) aslında bir pattern olarak karşımıza çıkar.
        enumerate fonksiyonu geriye iterasyondaki elemanın sıra numarası(index)
        ve değerini(value) döndürür.
        for (x,v) bu eşleşmeye bakar.
    */

    let market_list = vec![
        "Bir kilo prinç",
        "2 ekmek",
        "Yarım kilo un",
        "Bir paket dilimlenmiş kaşar peyniri",
        "Aç bitir salam",
    ]; // Evet bu kısımlarda acıkmışım

    for (x, v) in market_list.iter().enumerate() {
        println!("{} -> {}", x, v);
    }
}
