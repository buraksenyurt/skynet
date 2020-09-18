/*
    Pointer denilince aklımıza bellekteki bir bölgenin adresini tutan işaretçi gelir.
    Rust dilinde pointer'ların en bilinen tipi referans türleridir.
    Ancak birde Smart Pointer adı verilen veri yapısı vardır. Smart Pointer'da verinin adresini taşır ama
    ekstradan metadata bilgisi de barındırır.
    Hatta şu ana kadar birkaç smart pointer kullanmışızdır. String ve Vec veri yapıları Smart Pointer türündendir.
    Sadece rerenas adresi taşımazlar bununla birlikte ek bilgi de barındırırlar. Vector'ün başlangıç kapasitesi gibi.

    Bu arada pointer olarak belirtilen referans türleri veriyi ödünç alırlar(borrowing durumu)
    Aksina Smart Pointer'lar adreslenen veriyi sahiplenirler (ownership durumu)

    Bu örnekte smart pointer türlerini incelemeye çalışacaım.
*/

fn main() {
    /*
        En bilinen Smart Pointer türlerinden birisi Box<T>
        Veriyi Heap üzerinde tutmamızı sağlar. Tamam referans türü ile de bunu yapıyoruz ama metadata olayını unutmayalım.
        Ayrıca,
            Büyük bir verinin sahiplini kopyalamadan taşımak istediğimizde (Büyük veriyi Heap'te kutulayacağız)
            Derleme zamanında boyutunu bilemediğimiz bir veri kullandığımızda
        gibi durumlarda tercih edilir.
    */

    let a_number = Box::new(3.1415); // normalda stack'te duracak bir f32 verisini Heap'te kutuladık
    println!("{}", a_number);

    /*
        Rust, derleme zamanında tiplerin ne kadar yer tutacağını bilmek ister.
        Fonksiyonel dillerde rastlanan cons list gibi türler ise recursive özellik gösterirler ve tipin ne kadar yer tutacağı kestirilemez.
        cons Lisp kökenlidir ve iki argüman alan bir fonksiyondur. const list, cons fonksiyonunu recursive olarak çağıran bir elemanlar dizisidir.
        Son elemanda Nil görene kadar bu liste devam edebilir.
    */

    let infinity_war = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))); // Kafa karıştırıcı yahu
   
}

/*
        ConsList enum türüne bakalım. Cons fonksiyonunu ve Nil değerini içeriyor.
        Cons fonksiyonu da i32 türünden bir değer ve yine bir ConsList enum türü alıyor. İşte recursive veri yapısı.
        Bunu bu haliyle bırakırsak derleme zamanı 'recursive type `ConsList` has infinite size' şeklinde hata döner.
        O yüzden ConsListV2 şeklinde tanımlayıp kullanmamız gerekiyor.
*/
// enum ConsList {
//     Cons(i32, ConsList),
//     Nil,
// }

enum ConsListV2 {
    Cons(i32, Box<ConsListV2>), // Box kullandığımız için artık veriyi Heap'ta tutacağımızı belirttik.
    Nil,
}

use crate::ConsListV2::{Cons, Nil}; // Bu küfe bildirimini yapmazsak infinity_war kullanımında 'not found in this scope' hatası alırız
