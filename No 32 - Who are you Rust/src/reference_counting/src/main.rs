/*
    REFERENCE COUNTING - Rc<T> (RC diye bir kola vardı sanki bizamanlar...)

    Bir değerin birden fazla sahibi olduğu durumlar için geçerli bir konudur.
    Mesela bir graph ağacında bir boğumu işaret eden n sayıda boğum varsa, işaret edilen boğum için n sayıda sahiplikten söz edilebilir.
    Rc<T> aynı değeri işaret eden referanslar için muhasebeci görevini görür. Öyleki değeri işaret eden referansların sayısı sıfırsa o değer bellekten atılabilir.
*/

fn main() {
    println!("Hello, world!");
}
