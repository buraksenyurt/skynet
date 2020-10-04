/*
    Mutex...

    Rust dilinde kanallardan yararlanarak mesajlaşma yapan eş zamanlı thread'leri nasıl kullanabileceğimizi gördük.
    Kanallar (Channels) aslında tekil mülkiyet hakkı sağlarlar. Yani bir thread kanala bir veri bıraktığında bu veri onun için artık kullanılabilir değildir.
    Buna karşın birden fazla thread'in aynı anda aynı bellek bölgesini kullanmak isteyeceği durumlar da mümkündür.
    Bu tip durumların kanallar haricinde bir diğer yönetim şekli de Mutex tipini ele almaktır.
    Kanallar nasıl tekip mülkiyetliği baz alıyorsa, Mutex tipi de smart pointer'lar gibi çoklu mülkiyet/sahiplik hakkını baz alırlar(Keza Mutex<T> bir smart pointer'dır)
    Özetle çoklu thread'lerin aynı veriyi kullanmak istemeleri halinde t anında sadece bir tanesinin onu kullanmasına nasıl izin vereceğimizi görmek istiyorum.
    Mutex bu anlamda bakalım nasıl bir çözüm sunuyor görelim.

    Bir thread Mutex'e alınan bir veriyi kullanmak istediğinde bunu öncelikle ona sorar ve eğer müsaitse verinin kilidini(lock) alır. İşi bitince de
*/
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    /*
        #1

        İki thread Mutex kullanırken.
        worker_sue içinde "use of moved value" derleme zamanı hatası alırız.
        *collector satırında ise "borrow of moved value" derleme zamanı hatasını alırız.

    */
    // // Mutex nesnesini oluşturuyoruz. Tutacağı veri 32 bit integer ve değeri 1
    // let collector = Mutex::new(1);

    // // Bir thread başlatılıyor
    // let worker_joe = thread::spawn(move || {
    //     // mutex'in kilidini alıyoruz ve işimiz bitene kadar thread'i blokluyoruz
    //     let mut point = collector.lock().unwrap();
    //     // Burada kilidi bizde olan Mutex verisinin değerini değiştirdik
    //     *point += 5;
    // });
    // // Scope dışına çıktığımız anda Mutex'in kilidini de devretmiş olduk. Dolayısıyka başka bir thread artık bu kilidi alabilir.

    // /*
    //     Aşağıdaki satırda "use of moved value" hatası alınır
    //     Rust, collector'un mülkiyetinin birden fazla thread'e alınamayacağını söyler.
    //     Dikkat edin. Üst taraftaki thread için derleyici kızmaz. Aşağıdaki move || satırının altını çizerek kızar.
    // */
    // let worker_sue = thread::spawn(move || {
    //     let mut point = collector.lock().unwrap();
    //     *point += 3;
    // });

    // worker_joe.join().unwrap();
    // worker_sue.join().unwrap();

    // println!("{}", *collector.lock().unwrap());

    /*
        #2

        Üstteki kodda bildiğiniz üzere mülkiyet sorunu yaşadık.
        Thread Safe olarak Mutex'i diğer thread'lerin de kullanılmasını sağlamak için,
        Atomic Reference Counting Arc<T> tipinden yararlanabiliriz.

        Kodun çalışır hali aşağıdaki gibidir.

        Mutex verisini değiştirmek ve son halini almak için Arc üstünde klonladığımız referanslar olduğuna dikkat edelim.
    */
    let main_collector = Arc::new(Mutex::new(1));
    let mid_collector = Arc::clone(&main_collector);
    let last_collector = Arc::clone(&main_collector); // Buna neden ihtiyaç duydum?

    let worker_joe = thread::spawn(move || {
        let mut point = main_collector.lock().unwrap();
        *point += 5;
    });

    let worker_sue = thread::spawn(move || {
        let mut point = mid_collector.lock().unwrap();
        *point += 3;
    });

    worker_joe.join().unwrap();
    worker_sue.join().unwrap();

    println!("{}", *last_collector.lock().unwrap());
}
