/*
    REFERENCE COUNTING - Rc<T> (RC diye bir kola vardı sanki bizamanlar...)

    Bir değerin birden fazla sahibi olduğu durumlar için geçerli bir konudur.
    Mesela bir graph ağacında bir boğumu işaret eden n sayıda boğum varsa, işaret edilen boğum için n sayıda sahiplikten söz edilebilir.
    Rc<T> aynı değeri işaret eden referanslar için muhasebeci görevini görür. Öyleki değeri işaret eden referansların sayısı sıfırsa o değer bellekten atılabilir.
*/

use crate::PointList::{Cons, Nil};
use std::rc::Rc; // Rc<T> veri yapısını kullanabilmek için eklendi

fn main() {
    /*
        Konu Recursive veri yapılarından olan cons list kullanımı ile pek güzel ele alınıyor.
        points1, points2 ve points3 birer Cons List.
        points2 ve points3 oluşturulurken ilk değerler sonrası points1 listesine bağlanıyorlar.
        Hem points2 hem points3 aynı listeyi(points1) paylaşıyorlar.
        Esasında paylaşamıyorlar. points3 kısmında derleme zamanı hatası oluşuyor.
        Bu nedenle Box<T> smart pointer türü yerine Rc<T> türünü kullanmak gerekiyor.

        Bu arada hep bahsedilen referance count değerlerini görmek için strong_count fonksiyonunu nasıl kullandık bir bakalım.
        points1 ilk oluştuğunda bu değer 1 dir. points2, points2 points1'i kullanarak oluştuğunda bu değer 2ye çıkar.
        Sonrasında bir {} bloğu açıyoruz dikkat edilecek olursa. 
        points3 devreye girdiğinde sayaç 3e çıkar çünkü 3 referans söz konusudur.
        {} bloğundan sonra ise points3 scope dışı kalır ve dolayısıyla referance count 1 azalır.
    */

    // let points1 = Cons(7, Box::new(Cons(8, Box::new(Cons(9, Box::new(Nil)))))); //7->8->9->Nil şeklinde bir listemiz var
    // let points2 = Cons(1, Box::new(points1));
    // let points3 = Cons(3, Box::new(points1)); // Normalde bu şekilde kullanırsak, bir üst satırda points1'in sahipliği points2'ye geçtiği için use of moved value: `points1` derleme zamanı hatası alırız

    let points1 = Rc::new(Cons(7, Rc::new(Cons(8, Rc::new(Cons(9, Rc::new(Nil))))))); // Bir önceki kullanımdan farklı olarak Rc::new ile oluşturmaya başladığımıza dikkat edelim
    println!("Reference Count {}", Rc::strong_count(&points1));
    let points2 = Cons(1, Rc::clone(&points1)); // clone fonksiyonunu kullanarak points1'in referansını geçiyoruz
    {
        println!("Reference Count {}", Rc::strong_count(&points1));
        let points3 = Cons(3, Rc::clone(&points1));
        println!("Reference Count {}", Rc::strong_count(&points1));
    }
    println!("Reference Count {}", Rc::strong_count(&points1));
    // let points4 = Cons(10, points1.clone()); // Performans açısından tercih edilmez
    /*
        Bu arada Rc::clone(&points1) kullanımı yerine points1.clone() da tercih edilebilir ancak
        Rc::clone deep copy yapmadığından ve sadece referansmatiği (Counter diyelim) 1 artırdığından çok daha hızlı işlem görür.
    */
}

// // Kobay cons list yapımız
// enum PointList {
//     Cons(i32, Box<PointList>),
//     Nil,
// }

// Kobay cons list yapımız
enum PointList {
    Cons(i32, Rc<PointList>),
    Nil,
}
