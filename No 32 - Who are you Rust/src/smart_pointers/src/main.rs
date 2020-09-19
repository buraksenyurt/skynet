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

    /*
        Şimdi aşağıdaki kullanımlara bakalım.
        Normalde bir değişkenin referansını almak için & operatörünü kullanırız. Referans üstünden değer okurken de *(Dereference) operatörü kullanılır.
    */
    let mut point = 23;
    let pointer = &point;
    assert_eq!(23, *pointer); // * operatörünü kullanmadan denersek 'no implementation for `{integer} == &{integer}`' şeklinde hata alırız
                              // point = 25; // Bunu yapamayız. Çünkü borrowing söz konusudur. point, pointer tarafında referansla da olsa ödünç alınmıştır

    /*
       Yukarıdaki kullanımda pointer, point değerini referans eder.
       Aşağıdaki kullanımda ise counter değerinin bir kopyası Heap'e alınırken stack bölgesinden referans edilir.
    */
    let mut counter = 1;
    let smart_pointer = Box::new(counter);
    assert_eq!(1, *smart_pointer);
    counter += 1; // Bu mümkündür çünkü smart pointer borrowing durumunu oluşturmamıştır
                  //assert_eq!(2, *smart_pointer); // counter kendi başına artar. Smart Pointer onun değerini koypalayarak kullandığı için halen 1'e eşittir

    // Kendi smart pointer türümüzün kullanımı
    let lucky_num = 2.777;
    let magic_box = MagicBox::create(lucky_num);
    assert_eq!(2.777, *magic_box);
}

/*
    Kutsal Rustacean Kitabına göre Box yapısının referans türlerinden farkını anlamanı en iyi yolu kendi Smart Pointer türümüzü geliştirmekmiş.
    Tabii herhangi bir türle çalışması isteneceğininde generic tanımlanıyor.
    Tanımlayacağımız MagicBox yapısına create isimli bir fonksiyon da ekledik.(Box<T> türünün new fonksiyonu olarak düşünebiliriz)
    
    Ayrıca yukarıkdaki assert_eq!(2.777,*magic_box); satırında Deference operatörünün kullanımı söz konusu. 
    Bunu da kendi Smart Pointer yapımıza öğretmemiz gerekiyor. Aksi durumda 'type `MagicBox<{float}>` cannot be dereferenced' şeklinde derleme
    zamanı hatası alırız. Sonuçta oradaki kıyaslama için de * operatörü ile derefer ederek değeri almamız lazım.
*/
struct MagicBox<T>(T);

impl<T> MagicBox<T> {
    fn create(value: T) -> MagicBox<T> {
        MagicBox(value) // MagicBox bir Tuple gibi tasarlandığından onu metoda parametre olarak gelen value değeri ile oluşturuyoruz
    }
}

// Dereference için Deref Trait'inin uygulanması
use std::ops::Deref;

impl<T> Deref for MagicBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0 //Kara karıştırmasın. Tuple'ın ilk elemanının değerini döndürüyor
    }
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
