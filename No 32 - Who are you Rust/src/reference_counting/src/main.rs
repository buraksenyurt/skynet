/*
    REFERENCE COUNTING - Rc<T> (RC diye bir kola vardı sanki bizamanlar...)

    Bir değerin birden fazla sahibi olduğu durumlar için geçerli bir konudur.
    Mesela bir graph ağacında bir boğumu işaret eden n sayıda boğum varsa, işaret edilen boğum için n sayıda sahiplikten söz edilebilir.
    Rc<T> aynı değeri işaret eden referanslar için muhasebeci görevini görür. Öyleki değeri işaret eden referansların sayısı sıfırsa o değer bellekten atılabilir.
*/

use crate::PointList::{Cons, Nil};

fn main() {
    /*
        Konu Recursive veri yapılarından olan cons list kullanımı ile pek güzel ele alınıyor.
        points1, points2 ve points3 birer Cons List.
        points2 ve points3 oluşturulurken ilk değerler sonrası points1 listesine bağlanıyorlar.
        Hem points2 hem points3 aynı listeyi(points1) paylaşıyorlar.
        Esasında paylaşamıyorlar. points3 kısmında derleme zamanı hatası oluşuyor.
        Bu nedenle Box<T> smart pointer türü yerine Rc<T> türünü kullanmak gerekiyor.
    */

    // let points1 = Cons(7, Box::new(Cons(8, Box::new(Cons(9, Box::new(Nil)))))); //7->8->9->Nil şeklinde bir listemiz var
    // let points2 = Cons(1, Box::new(points1));
    // let points3 = Cons(3, Box::new(points1)); // Normalde bu şekilde kullanırsak, bir üst satırda points1'in sahipliği points2'ye geçtiği için use of moved value: `points1` derleme zamanı hatası alırız

    
}

// Kobay cons list yapımız
enum PointList {
    Cons(i32, Box<PointList>),
    Nil,
}
